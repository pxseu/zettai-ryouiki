use bytes::Bytes;
use serde::Deserialize;
use std::{collections::HashMap, io::Error};
use tokio::fs;

use super::utils;

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
        return Ok(());
    }

    let bytes = fetch_image(client, url, cookie).await.unwrap();

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

    let base_url = utils::get_base_path(illust.urls.original.clone());

    let ext = utils::get_ext(illust.urls.original.clone());

    let full_path = match base_path {
        Some(path) => format!("{}/{}", path, illust_id.clone()),
        None => illust_id.clone(),
    };

    fs::create_dir_all(full_path.clone()).await.unwrap();

    let mut downloads = Vec::new();

    for i in 0..=illust.page_count - 1 {
        let url = format!("{}/{}_p{}.{}", base_url, illust_id, i, ext);
        let path = format!("i_{}/p{}.{}", full_path, i, ext);

        let download = fetch_and_save(client.clone(), url, path, cookie.clone());
        downloads.push(download);
    }

    println!("Downloading {}...", full_path);

    futures::future::join_all(downloads)
        .await
        .into_iter()
        .for_each(|r| r.unwrap());

    println!("Done! {}", full_path);

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
            Some(format!("u_{}", user_id)),
            cookie.clone(),
        );
        illusts.push(task);
    }

    println!("Downloading {}'s images, total: {}", user_id, illusts.len());

    futures::future::join_all(illusts.into_iter().rev())
        .await
        .into_iter()
        .for_each(|r| r.unwrap());

    Ok(())
}
