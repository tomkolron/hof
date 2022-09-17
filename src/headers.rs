use reqwest::header;

#[tokio::main]

pub async fn get_headers(url: &'static str) -> Result<header::HeaderMap, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    Ok(client.get(url).send().await?.headers().clone())
}
