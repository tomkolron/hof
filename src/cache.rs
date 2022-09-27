use std::fs;
use std::io::Write;
use chrono::{Local, TimeZone};
use std::collections::HashMap;
use serde_json::{json, Value};

pub fn create_cache(hashmap: &HashMap<&str, String>) -> Result<(), Box<dyn std::error::Error>> {
    // println!("{}\n{}\n{}", hashmap["cookie"], hashmap["csrf"], hashmap["date"]);
    let mut cache_file = match fs::File::create(".cache") {
        Ok(file) => file,
        Err(error) => panic!("There was an error creating cache file: {}", error),
    };
    
    let json = json!({
        "cookie": hashmap["cookie"],
        "csrf": hashmap["csrf"],
        "date": hashmap["date"]
    });
    cache_file.write(json.to_string().as_bytes()).expect("");
    Ok(())
}

pub fn check_cache() -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>> {
    let cache_str = match fs::read_to_string(".cache") {
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
    let cache_json: Value = serde_json::from_str(&cache_str).expect("Error decoding cache file");
    let date = Local.datetime_from_str(cache_json["date"].as_str().unwrap(), "%Y-%m-%d %H:%M:%S %z").unwrap();
    let mut hashmap = HashMap::new();
    let mut cookie_str = cache_json["cookie"].to_string();
    cookie_str.remove(0);
    cookie_str.pop();
    let mut csrf_str = cache_json["csrf"].to_string();
    csrf_str.remove(0);
    csrf_str.pop();
    let mut date_str = cache_json["date"].to_string();
    date_str.remove(0);
    date_str.pop();

    if Local::now() < date {
        hashmap.insert("cookie", cookie_str);
        hashmap.insert("csrf", csrf_str);
        hashmap.insert("date", date_str);
    }else {
        hashmap.insert("none", String::from("true"));
    }
    Ok(hashmap)
}
