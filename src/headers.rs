use reqwest::{Client, header};
use core::time::Duration;
use progress_bar::*;
use std::collections::HashMap;
use std::time;

#[tokio::main]

pub async fn get_headers(urls: Vec<String>) -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>>{
    let mut headers: Vec<String> = Vec::new();
    
    let mut valid_urls: Vec<String> = Vec::new();

    let mut false_urls: Vec<String> = Vec::new();

    let mut hashmap: HashMap<&str, String> = HashMap::new();
     
    let mut times: Vec<u128> = Vec::new();
    let mut time_avg: u128 = 0;

    println!("Getting headers for all domains ...");

    init_progress_bar(urls.len());
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);
    
    let mut index: i32 = 0;

    for url in urls.iter() {
        let time = time::Instant::now();
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
        times.push(time.elapsed().as_millis());
        let time_sum: u128 = times.iter().sum();
        time_avg = time_sum / times.len() as u128;
        let time_remaining: f32 = ((urls.len() as i32 - index) * time_avg as i32) as f32 / 1000.0;
        // times_remaining.push(time_remaining * time_avg as i32);
        print_progress_bar_info("remaining: ", &time_remaining.to_string(), Color::Red, Style::Normal);
        // times_remaining.push(time_avg as i32);
        index += 1;
        inc_progress_bar();
    }

    finalize_progress_bar();
    // println!("{:?}", times_remaining);
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
