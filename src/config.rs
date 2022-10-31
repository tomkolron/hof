use std::fs;
use std::path::Path;
use serde_json::json;
use std::io::Write;

pub fn load_config() {
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

    // println!("file: {:?}", config_str);
}
