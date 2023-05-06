use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Urls {
    pub original: String,
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub error: bool,
    pub body: T,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Illust {
    pub page_count: u32,
    pub id: String,
    pub urls: Urls,
    pub illust_type: u32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub illusts: HashMap<String, Option<Illust>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ugoira {
    pub src: String,
    pub original_src: String,
    // pub mime_type: String,
    pub frames: Vec<UgoiraFrame>,
}

#[derive(Deserialize, Debug)]
pub struct UgoiraFrame {
    pub file: String,
    pub delay: u32,
}
