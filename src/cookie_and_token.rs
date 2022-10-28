use reqwest::Client;
use scraper::{Html, Selector};
use chrono::{Local, TimeZone};
use std::collections::HashMap;

#[tokio::main]

pub async fn get_cookie_and_token() -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>> {
    // Set url to get cookie and token from
    let url = "https://hackerone.com/reddit?type=team";
    
    // Create client
    let client = Client::new();

    // Make a request to the url
    let res = client.get(url).send().await?;

    // Get http headers of response
    let cookie_res = res.headers().clone();

    // Get html of response
    let csrf_res = res.text().await?.clone();

    // Parse csrf token out of html
    let csrf_doc = Html::parse_document(&csrf_res);
    let mut csrf = "";
    let selector = Selector::parse("meta[name=\"csrf-token\"]").unwrap();

    for i in csrf_doc.select(&selector) {
        csrf = i.value().attr("content").unwrap();
    }

    let mut headers_values: Vec<&str> = Vec::new();

    // Loop through cookies
    for value in cookie_res.values() {
        headers_values.push(value.to_str().unwrap());
    }

    let host: Vec<&str> = headers_values[9].split(";").collect();

    // Get the expire date
    let date = Local.datetime_from_str(host[2].replace(" expires=", "").as_str(), "%a, %d %h %Y %H:%M:%S GMT").unwrap();

    // Get the cookie 
    let cookie = host[0];

    let mut hashmap: HashMap<&str, String> = HashMap::new();
    hashmap.insert("cookie", cookie.to_string());
    hashmap.insert("csrf", csrf.to_string());
    hashmap.insert("date", date.to_string());

    // Return cookie, csrf token and expire date
    return Ok(hashmap);
}
