use reqwest::header::{HeaderMap, HeaderValue};

pub fn get_base_path(original_url: String) -> String {
    let mut base_url_split = original_url.split("/").collect::<Vec<&str>>();

    base_url_split.pop();
    base_url_split.clone().join("/")
}

pub fn get_ext(original_url: String) -> String {
    original_url
        .split(".")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap()
        .to_string()
}

pub fn get_headers(cookie: Option<String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "user-agent",
        HeaderValue::from_str(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36",
        ).unwrap(),
    );
    headers.insert("authority", HeaderValue::from_str("www.pixiv.net").unwrap());
    headers.insert(
        "referer",
        HeaderValue::from_str("https://www.pixiv.net/").unwrap(),
    );
    if cookie.is_some() {
        headers.insert(
            "cookie",
            HeaderValue::from_str(cookie.unwrap().as_str()).unwrap(),
        );
    }
    headers
}
