use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]

pub async fn get_cookie_and_token() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = "https://hackerone.com/reddit?type=team";
    let client = Client::new();
    let res = client.get(url).send().await?;
    let cookie_res = res.headers().clone();
    let csrf_res = res.text().await?.clone();
    let csrf_doc = Html::parse_document(&csrf_res);
    let mut csrf = "";
    let selector = Selector::parse("meta[name=\"csrf-token\"]").unwrap();

    for i in csrf_doc.select(&selector) {
        csrf = i.value().attr("content").unwrap();
    }

    let mut headers_keys: Vec<&str> = Vec::new();
    let mut headers_values: Vec<&str> = Vec::new();

    let mut host: Vec<&str> = Vec::new();

    for key in cookie_res.keys() {
        headers_keys.push(key.as_str());
    }
    for value in cookie_res.values() {
        headers_values.push(value.to_str().unwrap());
    }
    for i in 0..headers_keys.len() {
        if headers_keys[i] == "strict-transport-security" {
            host = headers_values[i].split(";").collect();
        }
    }
    let cookie = host[0];
    Ok(vec![cookie.to_string(), csrf.to_string()])
}
