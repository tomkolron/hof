use std::fs;
use std::path::Path;
use serde_json::{json, Value};
use std::io::Write;
use std::collections::HashMap;

pub fn load_config(config_path: &str) -> Result<HashMap<&'static str, String>, Box<dyn std::error::Error>> {
    // Create config file if it doesn't exist
    if Path::new(format!("{}/config.json", config_path).as_str()).exists() == false {
        let mut file = match fs::File::create(format!("{}/config.json", config_path)) {
            Ok(file) => file,
            Err(err) => panic!("Error create config file because: {}", err),
        };

        let config_default = json!({
            "use_vpn": false,
            "vpn_cmd": "",
            "vpn_loop": 25,
            "vpn_reconnect_delay": 10
        });

        file.write(serde_json::to_string_pretty(&config_default).unwrap().as_bytes()).expect("Couldn't write to config file");
    }

    let config_str = match fs::read_to_string(format!("{}/config.json", config_path)) {
            Ok(file) => file,
            Err(err) => panic!("Error reading cache file: {}", err),
    };

    let config_json: Value = serde_json::from_str(&config_str).expect("Error decoding config file"); 

    let use_vpn = config_json["use_vpn"].as_bool().unwrap(); 
    let vpn_cmd = config_json["vpn_cmd"].as_str().unwrap();
    let vpn_loop = config_json["vpn_loop"].as_i64().unwrap();
    let vpn_reconnect_delay = config_json["vpn_reconnect_delay"].as_i64().unwrap();

    let mut hash = HashMap::new();

    hash.insert("use_vpn", use_vpn.to_string());
    hash.insert("vpn_cmd", vpn_cmd.to_string());
    hash.insert("vpn_loop", vpn_loop.to_string());
    hash.insert("vpn_reconnect_delay", vpn_reconnect_delay.to_string());

    return Ok(hash);
}
