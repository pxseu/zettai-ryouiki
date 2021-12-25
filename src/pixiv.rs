use bytes::Bytes;
use serde::Deserialize;
use std::{collections::HashMap, io::Error};
use tokio::fs;

use super::{logger, utils};

#[derive(Deserialize, Debug)]
pub struct Urls {
    original: String,
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub error: bool,
    pub body: T,
}

#[derive(Deserialize, Debug)]
pub struct Illust {
    #[serde(rename = "pageCount")]
    pub page_count: u32,
    pub title: String,
    pub id: String,
    pub urls: Urls,
}

#[derive(Deserialize, Debug)]
pub struct User {
    illusts: HashMap<String, Option<Illust>>,
}

async fn get_illust(
    client: reqwest::Client,
    illust_id: String,
    cookie: Option<String>,
) -> Result<Illust, Error> {
    let url = format!("https://www.pixiv.net/ajax/illust/{}", illust_id);
    let res = client
        .get(url)
        .headers(utils::get_headers(cookie))
        .send()
        .await
        .unwrap();
    let res = res.json::<Response<Illust>>().await.unwrap();

    Ok(res.body)
}

async fn fetch_image(
    client: reqwest::Client,
    url: String,
    cookie: Option<String>,
) -> Result<Bytes, Error> {
    let res = client
        .get(url.clone())
        .headers(utils::get_headers(cookie.clone()))
        .send()
        .await
        .unwrap();

    let result = res.bytes().await.unwrap();

    Ok(result)
}

async fn fetch_and_save(
    client: reqwest::Client,
    url: String,
    path: String,
    cookie: Option<String>,
) -> Result<(), Error> {
    if fs::metadata(path.clone()).await.is_ok() {
        logger::log(format!("{} already exists", path));
        return Ok(());
    }

    logger::log(format!("Downloading {}", path));

    let bytes = fetch_image(client, url, cookie).await.unwrap();

    logger::log(format!("Saving {}", path));

    fs::write(path, bytes).await?;

    Ok(())
}

pub async fn download_image(
    client: reqwest::Client,
    illust_id: String,
    base_path: Option<String>,
    cookie: Option<String>,
) -> Result<(), Error> {
    let illust = get_illust(client.clone(), illust_id.clone(), cookie.clone())
        .await
        .unwrap();

    logger::log(format!(
        "Fetching images of {} ({} pages)",
        illust_id, illust.page_count
    ));

    let base_url = utils::get_base_path(illust.urls.original.clone());

    let ext = utils::get_ext(illust.urls.original.clone());

    let full_path = match base_path {
        Some(path) => format!("u_{}/i_{}", path, illust_id.clone()),
        None => format!("i_{}", illust_id.clone()),
    };

    fs::create_dir_all(full_path.clone()).await.unwrap();

    let mut downloads = Vec::new();

    for i in 0..=illust.page_count - 1 {
        let url = format!("{}/{}_p{}.{}", base_url, illust_id, i, ext);
        let path = format!("{}/p{}.{}", full_path, i, ext);

        let download = fetch_and_save(client.clone(), url, path, cookie.clone());
        downloads.push(download);
    }

    futures::future::join_all(downloads)
        .await
        .into_iter()
        .for_each(|r| r.unwrap());

    logger::log(format!("Done! {}", full_path));

    Ok(())
}

pub async fn download_user(
    client: reqwest::Client,
    user_id: String,
    cookie: Option<String>,
) -> Result<(), Error> {
    let url = format!(
        "https://www.pixiv.net/ajax/user/{}/profile/all",
        user_id.clone()
    );

    let res = client
        .get(url)
        .headers(utils::get_headers(cookie.clone()))
        .send()
        .await
        .unwrap();
    let res = res.json::<Response<User>>().await.unwrap();

    let mut illusts = Vec::new();

    for (key, _) in res.body.illusts.iter() {
        let task = download_image(
            client.clone(),
            key.to_string(),
            Some(user_id.clone()),
            cookie.clone(),
        );
        illusts.push(task);
    }

    logger::log(format!(
        "Downloading {}'s images, total: {}",
        user_id,
        illusts.len()
    ));

    futures::future::join_all(illusts.into_iter().rev())
        .await
        .into_iter()
        .for_each(|r| r.unwrap());

    Ok(())
}
