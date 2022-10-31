use std::fs;
use std::path::Path;
use serde_json::{json, Value};
use std::io::Write;
use std::collections::HashMap;

pub fn load_config() -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>> {
    // Create config dir if it doesn't exist
    fs::create_dir_all("/home/tom/.config/hof").unwrap();

    // Create config file if it doesn't exist
    if Path::new("/home/tom/.config/hof/config.txt").exists() == false {
        let mut file = match fs::File::create("/home/tom/.config/hof/config.txt") {
            Ok(file) => file,
            Err(err) => panic!("Error create config file because: {}", err),
        };

        let config_default = json!({
            "use_vpn": false,
            "vpn_cmd": "",
            "vpn_loop": 25
        });

        file.write(serde_json::to_string_pretty(&config_default).unwrap().as_bytes()).expect("Couldn't write to cach");
    }

    let config_str = match fs::read_to_string("/home/tom/.config/hof/config.txt") {
            Ok(file) => file,
            Err(err) => panic!("Error reading cache file: {}", err),
    };

    let config_json: Value = serde_json::from_str(&config_str).expect("Error decoding config file"); 
    if config_json["use_vpn"].as_bool().unwrap() == true && config_json["vpn_cmd"].as_str().unwrap() != "" {
        let vpn_cmd = config_json["vpn_cmd"].as_str().unwrap();
        let vpn_loop = config_json["vpn_loop"].as_i64().unwrap();

        let mut hash = HashMap::new();

        hash.insert("vpn_cmd", vpn_cmd.to_string());
        hash.insert("vpn_loop", vpn_loop.to_string());
        return Ok(hash);
    }else {
        let mut hash = HashMap::new();
        hash.insert("disable", String::from("true"));
        return Ok(hash);
    }
}
