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
