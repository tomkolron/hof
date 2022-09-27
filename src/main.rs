mod args;
mod scopes;
mod bounties;
mod subdomains;
mod headers;
mod cookie_and_token;

use std::{fs, io, process, time};
use std::io::Write;
use clap::Parser;

use args::FbbArgs;
use bounties::get_bounties;
use scopes::get_scopes;
use subdomains::get_subs;
use headers::get_headers;
use cookie_and_token::get_cookie_and_token;

use stybulate::{Table, Style, Cell, Headers};

fn main() {
    // Set time to measure how long program runs
    let time = time::Instant::now();

    // Get cli argumets
    let args = FbbArgs::parse();

    // Set path
    let path = format!("{}/{}", args.path.clone(), args.query.clone());

    // Create directory
    println!("Creating project directory ...\n");
    let dir = fs::create_dir(path.clone());
    match dir {
        Ok(()) => {},
        Err(error) => match error.kind() {
            io::ErrorKind::AlreadyExists => overwrite_directory(path.clone()),
            other_error => panic!("Problem creating directory: {:?}", other_error),
        }
    }


    // Create scopes file
    let mut scopes_file = fs::File::create(format!("{}/scopes.txt", path.clone())).expect("Error creating scopes file");

    // Filter scopes
    let mut filtered_scopes = Vec::new();

    // Subs scopes
    let mut subs_scopes = Vec::new();

    // Get cookie and csrf token
    let cookie_and_token = get_cookie_and_token();

    // Get domain scopes
    println!("Writing to scopes file:\n");
    let scopes = get_scopes(args.query.clone(), &cookie_and_token.as_ref().unwrap()[0].clone(), &cookie_and_token.as_ref().unwrap()[1].clone());
    for scope in scopes.as_ref().unwrap().iter() {
        println!("{}", scope);

        let file_scope = format!("{}\n", scope);

        let filtered_scope = scope.replace("*.", "");
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
        all_domains.push(String::from(format!("https://{}", scope)));
    }

    if subs_scopes.len() > 0 {
        // Create subdomains file
        let mut subs_file = fs::File::create(format!("{}/subdomains.txt", path.clone())).expect("Error creating subdomains file");
        
        // Get subdomains
        let subs = get_subs(subs_scopes.clone());
        let subs_vec: Vec<&str> = subs.split("\n").collect();
        subs_file.write(subs.as_bytes()).expect("Error writing to subdomains file");
        println!("Found {} subdomains\n", subs_vec.len() - 1);
        for sub in &subs_vec {
            all_domains.push(String::from(format!("https://{}", sub)));
        }
    }
   
    // Create valid urls file
    let mut valid_urls_file = fs::File::create(format!("{}/valid_urls.txt", path.clone())).expect("Error creating valid urls file");

    // Create false urls file
    let mut false_urls_file = fs::File::create(format!("{}/false_urls.txt", path.clone())).expect("Error creating false urls file");

    // Create http headers file
    let mut headers_file = fs::File::create(format!("{}/headers.txt", path.clone())).expect("Error creating http headers file");

    // Get headers
    let headers = get_headers(all_domains);
    // println!("{:?}", headers);
    headers_file.write(headers.as_ref().unwrap()["headers"].as_bytes()).expect("Error writing to http headers file");
    valid_urls_file.write(headers.as_ref().unwrap()["valid_urls"].as_bytes()).expect("Error writing to valid domains file");
    false_urls_file.write(headers.as_ref().unwrap()["false_urls"].as_bytes()).expect("Error writing to false domains file");


    // Print http headers statistics 
    let mut valid_urls_count: Vec<&str> = headers.as_ref().unwrap()["valid_urls"].split("\n").collect();
    valid_urls_count.pop();

    let mut false_urls_count: Vec<&str> = headers.as_ref().unwrap()["false_urls"].split("\n").collect();
    false_urls_count.pop();

    println!("\nFound {} valid urls and {} false urls.\n", valid_urls_count.len(), false_urls_count.len());

    // Get bounties
    let bounties = get_bounties(args.query.clone());
    if bounties.as_ref().unwrap()[0] == "none" {
        println!("No bounty reward avalible.");

    }else {
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
        println!("\n{}", bounty_table);
    }

    let duration = time.elapsed();
    let millis = duration.as_millis() % 60;
    let seconds = (duration.as_millis() / 1000) % 60;
    let minutes = (duration.as_millis() / 1000) / 60;
    println!("took {}m and {}.{}s to run.", minutes, seconds, millis);
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
