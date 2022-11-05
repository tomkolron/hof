use reqwest::{Client, header};
use core::time::Duration;
use std::collections::HashMap;
use indicatif::{ProgressBar,ProgressStyle};
use std::process::Command;
use std::{thread, time};

#[tokio::main]

pub async fn get_headers(urls: Vec<String>, timeout: u64, config: HashMap<&str, String>) -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>>{
    // Get config
    let vpn_loop = config["vpn_loop"].clone().parse::<u8>().unwrap();
    let vpn_reconnect_delay = time::Duration::from_secs(config["vpn_reconnect_delay"].clone().parse::<u64>().unwrap());
    let vpn_cmd = &config["vpn_cmd"];

    let mut cmd_index = 0;
    let mut args: Vec<&str> = Vec::new();
    let mut command:String = String::from("");

    for i in vpn_cmd.split(' ') {
        if cmd_index == 0 {
            command = String::from(i);
        }else {
            args.push(i);
        }
        cmd_index += 1;
    }

    // Set vpn counter
    let mut vpn_counter = 0;

    // Declare progress bar variable
    let progress = ProgressBar::new(urls.len().try_into().unwrap());

    // Style progress bar
    progress.set_style(
        ProgressStyle::with_template(
            "[{elapsed}] [{bar:60.green}] {pos}/{len}, ETA {eta}{msg}",
        )
        .unwrap()
        .progress_chars("=> ")
    );

    // Crate empty vectors
    let mut headers: Vec<String> = Vec::new();
    
    let mut valid_urls: Vec<String> = Vec::new();

    let mut false_urls: Vec<String> = Vec::new();

    let mut hashmap: HashMap<&str, String> = HashMap::new();
     
    println!("Getting headers for all domains ...");

    // Itirate through all the urls
    for url in urls.iter() {
        // Check if url is valid
        match make_req(url.to_string(), timeout).await {
            Ok(res) => {
                // Push title to headers vec
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
                    // Push http header to headers vec
                    headers.push(format!("{}: {}\n", headers_keys[i], headers_values[i]));
                }
            },
            // If url is not valid
            Err(_error) => {
                // Push url to false_urls vec
                false_urls.push(String::from(format!("{}\n", url)));
            },

        }
        // Check vpn counter
        if vpn_counter == vpn_loop {
            // Check if vpn is enabled in config
            if config["use_vpn"] == String::from("true") {
                // Print message to progress bar when reconnecting vpn
                progress.set_message(", reconnecting vpn");

                // Reconnect vpn
                Command::new(&command)
                    .args(&args)
                    .output()
                    .expect("failed to reconnect vpn");

                // Sleep specifed amount
                thread::sleep(vpn_reconnect_delay);

                // Remove message from progress bar
                progress.set_message("");
            }

            // Reset vpn counter
            vpn_counter = 0;
        }

        // Increment vpn counter
        vpn_counter += 1;

        // Increment the progress bar
        progress.inc(1);

    }
    // Finish progress bar
    progress.finish();
    println!("");

    // Return header, valid_url, false_urls
    hashmap.insert("headers", headers.join(""));
    hashmap.insert("valid_urls", valid_urls.join(""));
    hashmap.insert("false_urls", false_urls.join(""));
    return Ok(hashmap);
}

// Function to make requests
async fn make_req(url: String, timeout: u64) -> Result<header::HeaderMap, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(url).timeout(Duration::from_secs(timeout)).send().await?.headers().clone();
    return Ok(res);
}
