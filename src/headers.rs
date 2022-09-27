use reqwest::{Client, header};
use core::time::Duration;
use progress_bar::*;
use std::collections::HashMap;

#[tokio::main]

pub async fn get_headers(urls: Vec<String>) -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>>{
    let mut headers: Vec<String> = Vec::new();
    
    let mut valid_urls: Vec<String> = Vec::new();

    let mut false_urls: Vec<String> = Vec::new();

    let mut hashmap: HashMap<&str, String> = HashMap::new();

    println!("Getting headers for all domains ...");

    init_progress_bar(urls.len());
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);

    for url in urls.iter() {
        match make_req(url.to_string()).await {
            Ok(res) => {
                valid_urls.push(String::from(format!("{}\n", url)));

                headers.push(String::from(format!("\nUrl: {}\n\n", url)));

                let mut headers_keys: Vec<&str> = Vec::new();
                let mut headers_values: Vec<&str> = Vec::new();
                
                for key in res.keys() {
                    headers_keys.push(key.as_str());
                }
                for value in res.values() {
                    headers_values.push(value.to_str().unwrap());
                }
                for i in 0..headers_keys.len() {
                    headers.push(format!("{}: {}\n", headers_keys[i], headers_values[i]));
                }
            },
            Err(_error) => {
                false_urls.push(String::from(format!("{}\n", url)));
            },
        }
        inc_progress_bar();
    }

    finalize_progress_bar();
    hashmap.insert("headers", headers.join(""));
    hashmap.insert("valid_urls", valid_urls.join(""));
    hashmap.insert("false_urls", false_urls.join(""));
    return Ok(hashmap);
    // return Ok(vec![headers.join(""), valid_urls.join(""), false_urls.join("")]);
}

async fn make_req(url: String) -> Result<header::HeaderMap, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(url).timeout(Duration::from_secs(8)).send().await?.headers().clone();
    return Ok(res);
}
