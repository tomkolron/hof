use reqwest::{Client, header};
use core::time::Duration;

#[tokio::main]

pub async fn get_headers(urls: Vec<String>) -> Result<String, Box<dyn std::error::Error>>{
    let mut headers: Vec<String> = Vec::new();

    for url in urls.iter() {
        match make_req(url.to_string()).await {
            Ok(res) => {
                let mut headers_keys: Vec<&str> = Vec::new();
                let mut headers_values: Vec<&str> = Vec::new();
                headers.push(String::from("\n"));
                headers.push(String::from(format!("Url: {}\n", url)));
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
            Err(_error) => println!("False url found: {}", url),
        }
    }

    return Ok(headers.join(""));
}

async fn make_req(url: String) -> Result<header::HeaderMap, Box<dyn std::error::Error>> {
    println!("Getting http response headers for {} ...", url);
    let client = Client::new();
    let res = client.get(url).timeout(Duration::from_secs(8)).send().await?.headers().clone();
    return Ok(res);
}
