use std::process::Command;

pub fn get_subs(scopes: Vec<String>) -> String {
    let mut subdomains = Vec::new();
    for scope in scopes.iter() {
        println!("Getting subdomains for: {}", scope);
        let mut cmd = Command::new("findomain");
        cmd.args(["-q", "-t", scope]);
        match cmd.output() {
            Ok(output) => {
                let output_final = String::from_utf8(output.stdout).expect("Error formatting command output");
                let mut output_final_clean = output_final.clone();
                output_final_clean.pop();
                output_final_clean.remove(0);
                subdomains.push(output_final_clean);
                println!("{}", output_final.clone());
            },
            Err(error) => panic!("Error getting subdomain: {:?}", error),
        }
    }
    let subdomains_joined = subdomains.join("");
    subdomains_joined
}
