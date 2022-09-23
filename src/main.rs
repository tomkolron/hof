mod args;
mod scopes;
mod bounties;
mod subdomains;
mod headers;

use std::{fs, io, process, time};
use std::io::Write;
use clap::Parser;

use args::FbbArgs;
use bounties::get_bounties;
use scopes::get_scopes;
use subdomains::get_subs;
use headers::get_headers;

use stybulate::{Table, Style, Cell, Headers};

fn main() {
    // Set time to measure how long program runs
    let time = time::Instant::now();

    // Get cli argumets
    let args = FbbArgs::parse();

    // Create directory
    println!("Creating project directory ...\n");
    let dir = fs::create_dir(args.path.clone());
    match dir {
        Ok(()) => {},
        Err(error) => match error.kind() {
            io::ErrorKind::AlreadyExists => overwrite_directory(args.path.clone()),
            other_error => panic!("Problem creating directory: {:?}", other_error),
        }
    }


    // Create scopes file
    let mut scopes_file = fs::File::create(format!("{}/scopes.txt", args.path.clone())).expect("Error creating scopes file");

    // Filter scopes
    let mut filtered_scopes = Vec::new();

    // Subs scopes
    let mut subs_scopes = Vec::new();

    // Get domain scopes
    println!("Writing to scopes file:\n");
    let scopes = get_scopes(args.query.clone());
    for scope in scopes.as_ref().unwrap().iter() {
        println!("{}", scope);

        let file_scope = format!("{}\n", scope);

        let filtered_scope = scope.replace("*.", "https://");
        filtered_scopes.push(filtered_scope);

        if scope.contains("*.") {
            let subs_scope = scope.replace("*.", "");
            subs_scopes.push(subs_scope);
        }
        scopes_file.write(file_scope.as_bytes()).expect("Error writing to scopes file");
    }
    println!("");


    // Set all_domains
    let mut all_domains: Vec<String> = Vec::new();
    for scope in filtered_scopes {
        all_domains.push(String::from(scope));
    }

    if subs_scopes.len() > 0 {
        // Create subdomains file
        let mut subs_file = fs::File::create(format!("{}/subdomains.txt", args.path.clone())).expect("Error creating subdomains file");
        
        // Get subdomains
        let subs = get_subs(subs_scopes.clone());
        let subs_vec: Vec<&str> = subs.split("\n").collect();
        subs_file.write(subs.as_bytes()).expect("Error writing to subdomains file");
        println!("Found {} subdomains\n", subs_vec.len() - 1);
        for sub in &subs_vec {
            all_domains.push(String::from(format!("https://{}", sub)));
        }
    }
   

    // Create http headers file
    let mut headers_file = fs::File::create(format!("{}/headers.txt", args.path.clone())).expect("Error creating subdomains file");

    // Get headers
    let headers = get_headers(all_domains);
    headers_file.write(headers.unwrap().as_bytes()).expect("Error writing to http headers file");


    // Get bounties
    let bounties = get_bounties(args.query.clone());
    let bounty_table = Table::new(
        Style::Fancy,
        vec![
            vec![Cell::from("Low"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[0]).as_str())],
            vec![Cell::from("Medium"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[1]).as_str())],
            vec![Cell::from("High"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[2]).as_str())],
            vec![Cell::from("Critical"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[3]).as_str())],
        ],
        Some(Headers::from(vec!["bounty", "prize"])),
    ).tabulate();
    println!("{}", bounty_table);
    println!("took {}s to run.", time.elapsed().as_secs());
}

fn overwrite_directory(path: String) {
    println!("Directory already exist would you like to overwrite it [y, N]");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).expect("Problem reading user input");
    if user_input == "y\n" || user_input == "Y\n" {
        println!("overwriting directory ...\n");
        fs::remove_dir_all(&path).expect("There was a problem overwriting directory");
        fs::create_dir(&path).expect("There was a problem overwriting directory");
    }else {
        quit()
    }
}

fn quit() {
    println!("GoodBye \u{1f44b}");
    process::abort();
}
