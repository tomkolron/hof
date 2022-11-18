use std::fs;
use std::io::Write;
use chrono::{Local, TimeZone};
use std::collections::HashMap;
use serde_json::{json, Value};

pub fn create_cache(hashmap: &HashMap<&str, String>, cache_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create cache file
    let mut cache_file = match fs::File::create(format!("{}/cache", cache_path)) {
        Ok(file) => file,
        Err(error) => panic!("There was an error creating cache file: {}", error),
    };
    
    // Set json for chache
    let json = json!({
        "cookie": hashmap["cookie"],
        "csrf": hashmap["csrf"],
        "date": hashmap["date"]
    });

    // Write json to cache file
    cache_file.write(serde_json::to_string_pretty(&json).unwrap().as_bytes()).expect("Couldn't write to cach");

    // Return OK
    return Ok(());
}

pub fn check_cache(cache_path: &str) -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>> {
    // Read from the cache file
    let cache_str = match fs::read_to_string(format!("{}/cache", cache_path)) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                let mut hashmap = HashMap::new();
                hashmap.insert("none", String::from("true"));
                return Ok(hashmap);
            },
            _ => panic!("Error reading cache file"),
        },
    };

    // Get json from file
    let cache_json: Value = serde_json::from_str(&cache_str).expect("Error decoding cache file");

    // Get date out of json
    let date = Local.datetime_from_str(cache_json["date"].as_str().unwrap(), "%Y-%m-%d %H:%M:%S %z").unwrap();
    let mut hashmap = HashMap::new();

    // Check if cache expired
    if Local::now() < date {
        hashmap.insert("cookie", cache_json["cookie"].as_str().unwrap().to_string());
        hashmap.insert("csrf", cache_json["csrf"].as_str().unwrap().to_string());
        hashmap.insert("date", cache_json["date"].as_str().unwrap().to_string());
    }else {
        hashmap.insert("none", String::from("true"));
    }

    // Return nothing if cache expired and return cache if it didn't
    return Ok(hashmap);
}
