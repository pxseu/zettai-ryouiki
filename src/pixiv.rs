use anyhow::{anyhow, bail, Context, Result};
use tokio::fs;

use super::types::{Illust, Response, Ugoira, User};
use super::{logger, utils};

const BASE_HOSTNAME: &str = "www.pixiv.net";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                          AppleWebKit/537.36 (KHTML, like Gecko) \
                          Chrome/80.0.3987.132 Safari/537.36";

#[derive(Debug, Clone)]
pub struct Pixiv {
    client: reqwest::Client,
}

impl Default for Pixiv {
    fn default() -> Self {
        Self::new(None).unwrap()
    }
}

impl Pixiv {
    pub fn new(cookie: Option<String>) -> Result<Self> {
        use reqwest::header::{HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();

        headers.insert("user-agent", HeaderValue::from_str(USER_AGENT)?);
        headers.insert("authority", HeaderValue::from_str(BASE_HOSTNAME)?);
        headers.insert(
            "referer",
            HeaderValue::from_str(&format!("https://{BASE_HOSTNAME}/"))?,
        );

        if let Some(cookie) = cookie {
            headers.insert("cookie", HeaderValue::from_str(cookie.as_str())?);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Pixiv { client })
    }

    pub async fn download_image(&self, illust_id: &str, author: Option<String>) -> Result<()> {
        let illust = self.get_illust(illust_id.clone()).await?;

        logger::log(format!(
            "Fetching images of {illust_id} ({} pages)",
            illust.page_count
        ));

        let base_url = utils::get_base_path(illust.urls.original.clone());

        let ext = utils::get_ext(illust.urls.original.clone());

        let full_path = match author {
            Some(path) => format!("u_{path}/i_{illust_id}"),
            None => format!("i_{illust_id}"),
        };

        fs::create_dir_all(full_path.clone()).await?;

        match illust.illust_type {
            // map each page to a future that downloads the image
            0 => {
                let downloads = (0..=illust.page_count - 1).into_iter().map(|page| {
                    self.fetch_and_save_image(
                        format!("{base_url}/{illust_id}_p{page}.{ext}"),
                        format!("{full_path}/p{page}.{ext}"),
                    )
                });

                for task in futures::future::join_all(downloads).await.into_iter() {
                    if let Err(e) = task {
                        logger::log(format!("{e:?}"));
                    }
                }
            }

            2 => {
                let (first, second) = tokio::join!(
                    self.fetch_and_save_image(
                        illust.urls.original,
                        format!("{full_path}/thumbnail.{ext}"),
                    ),
                    self.fetch_and_save_ugoira(
                        illust_id,
                        format!("{full_path}/{illust_id}_ugoira1920x1080.zip"),
                    ),
                );

                first?;
                second?;
            }

            _ => bail!("Unsupported illust type: {}, skipping", illust.illust_type),
        };

        logger::log(format!("Done! {full_path}"));

        Ok(())
    }

    pub async fn download_user(&self, user_id: &str) -> Result<()> {
        let res = self
            .client
            .get(format!(
                "https://www.pixiv.net/ajax/user/{user_id}/profile/all",
            ))
            .send()
            .await?
            .error_for_status()?
            .json::<Response<User>>()
            .await?;

        let illusts = res
            .body
            .illusts
            .keys()
            .map(|illust_id| self.download_image(illust_id, Some(user_id.to_string())));

        logger::log(format!(
            "Downloading {user_id}'s images, total: {}",
            illusts.len()
        ));

        for task in futures::future::join_all(illusts).await.into_iter() {
            if let Err(e) = task {
                logger::log(format!("{e:?}"));
            }
        }

        Ok(())
    }

    async fn fetch_and_save_ugoira(&self, ugoira_id: &str, path: String) -> Result<()> {
        let data = self.get_ugoira_meta(ugoira_id).await?;

        self.fetch_and_save_image(data.original_src, path)
            .await
            .context("Failed to download ugoira zip")
    }

    async fn fetch_and_save_image(&self, url: String, path: String) -> Result<()> {
        if fs::metadata(&path).await.is_ok() {
            logger::log(format!("{path} already exists, skipping"));
            return Ok(());
        }

        logger::log(format!("Downloading {path}"));

        let bytes = self.fetch_bytes(&url).await?;

        logger::log(format!("Saving {path}"));

        fs::write(path, bytes).await?;

        Ok(())
    }

    async fn get_ugoira_meta(&self, ugoira_id: &str) -> Result<Ugoira> {
        let data = self
            .client
            .get(format!(
                "https://www.pixiv.net/ajax/illust/{ugoira_id}/ugoira_meta?lang=en",
            ))
            .send()
            .await?
            .error_for_status()?
            .json::<Response<Ugoira>>()
            .await?
            .body;

        Ok(data)
    }

    async fn get_illust(&self, illust_id: &str) -> Result<Illust> {
        let data = self
            .client
            .get(format!("https://www.pixiv.net/ajax/illust/{illust_id}"))
            .send()
            .await?
            .error_for_status()?
            .json::<Response<Illust>>()
            .await
            .with_context(|| anyhow!("Failed to get illust: {illust_id}"))?;

        Ok(data.body)
    }

    async fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>> {
        let data = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(data.to_vec())
    }
}
