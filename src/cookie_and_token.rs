use reqwest::Client;
use scraper::{Html, Selector};
use chrono::{Local, TimeZone};
use std::collections::HashMap;

#[tokio::main]

pub async fn get_cookie_and_token() -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>> {
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

    let mut headers_values: Vec<&str> = Vec::new();

    for value in cookie_res.values() {
        headers_values.push(value.to_str().unwrap());
    }
    let host: Vec<&str> = headers_values[9].split(";").collect();
    let date = Local.datetime_from_str(host[2].replace(" expires=", "").as_str(), "%a, %d %h %Y %H:%M:%S GMT").unwrap();

    let cookie = host[0];

    let mut hashmap: HashMap<&str, String> = HashMap::new();
    hashmap.insert("cookie", cookie.to_string());
    hashmap.insert("csrf", csrf.to_string());
    hashmap.insert("date", date.to_string());

    Ok(hashmap)
}
