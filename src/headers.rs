use reqwest::{Client, header};
use core::time::Duration;
use std::collections::HashMap;
use indicatif::{ProgressBar,ProgressStyle};

#[tokio::main]

pub async fn get_headers(urls: Vec<String>) -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>>{
    //Declare progress bar variable
    let progress = ProgressBar::new(urls.len().try_into().unwrap());

    //Style progress bar
    progress.set_style(
        ProgressStyle::with_template(
            "[{elapsed}] [{bar:60.green}] {pos}/{len}, ETA {eta}",
        )
        .unwrap()
        .progress_chars("=> ")
    );

    //Crate empty vectors
    let mut headers: Vec<String> = Vec::new();
    
    let mut valid_urls: Vec<String> = Vec::new();

    let mut false_urls: Vec<String> = Vec::new();

    let mut hashmap: HashMap<&str, String> = HashMap::new();
     
    println!("Getting headers for all domains ...");

    //Itirate through all the urls
    for url in urls.iter() {
        //Check if url is valid
        match make_req(url.to_string()).await {
            Ok(res) => {
                //Push title to headers vec
                if url == &urls[0] {
                    headers.push(String::from(format!("Url: {}\n\n", url)));
                }else {
                    headers.push(String::from(format!("\nUrl: {}\n\n", url)));
                }

                valid_urls.push(String::from(format!("{}\n", url)));


                let mut headers_keys: Vec<&str> = Vec::new();
                let mut headers_values: Vec<&str> = Vec::new();
                
                for key in res.keys() {
                    headers_keys.push(key.as_str());
                }

                for value in res.values() {
                    headers_values.push(value.to_str().unwrap());
                }

                for i in 0..headers_keys.len() {
                    //Push http header to headers vec
                    headers.push(format!("{}: {}\n", headers_keys[i], headers_values[i]));
                }
            },
            //If url is not valid
            Err(_error) => {
                //Push url to false_urls vec
                false_urls.push(String::from(format!("{}\n", url)));
            },
        }
        //Increment the progress bar
        progress.inc(1);
    }
    //Finish progress bar
    progress.finish();
    println!("");

    //Return header, valid_url, false_urls
    hashmap.insert("headers", headers.join(""));
    hashmap.insert("valid_urls", valid_urls.join(""));
    hashmap.insert("false_urls", false_urls.join(""));
    return Ok(hashmap);
}

//Function to make requests
async fn make_req(url: String) -> Result<header::HeaderMap, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(url).timeout(Duration::from_secs(8)).send().await?.headers().clone();
    return Ok(res);
}
