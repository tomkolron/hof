use std::process::Command;
use std::io::ErrorKind;

pub fn get_subs(scopes: Vec<String>) -> String {
    // Declare empty subdomains vector
    let mut subdomains = Vec::new();

    // Loop through all domain scopes
    for scope in scopes.iter() {
        println!("Getting subdomains for: {}", scope);

        // Declare and run command to find subdomains
        let mut cmd = Command::new("findomain");
        cmd.args(["-q", "-t", scope]);

        // Check if command ran seccusfully
        match cmd.output() {
            Ok(output) => {
                // Get output
                let mut output_final = String::from_utf8(output.stdout).expect("Error formatting command output");

                // Remove clutter from output
                output_final.remove(0);
                output_final.pop();

                // Print output
                println!("\n{}\n", output_final.clone());

                // Push output to subdomains vec
                subdomains.push(output_final);
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("You need to install findomain to use this app, instruction here: https://github.com/Findomain/Findomain"),
                other_error => panic!("Error getting subdomain: {:?}", other_error),
            },
        }
    }

    // Return all subdomains joined
    return subdomains.join("");
}
