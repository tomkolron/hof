mod args;
mod scopes;
mod bounties;
mod subdomains;

use std::{fs, io, process};
use std::io::Write;
use clap::Parser;

use args::FbbArgs;
use bounties::get_bounties;
use scopes::get_scopes;
use subdomains::get_subs;

use stybulate::{Table, Style, Cell, Headers};

fn main() {
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

    // Get domain scopes
    println!("Writing to scopes file:\n");
    let scopes = get_scopes(args.query.clone());
    for scope in scopes.as_ref().unwrap().iter() {
        println!("{}", scope);
        let file_scope = format!("{}\n", scope);
        let filtered_scope = scope.replace("*.", "");
        filtered_scopes.push(filtered_scope);
        scopes_file.write(file_scope.as_bytes()).expect("Error writing to scopes file");
    }
    println!("");

    // Create subdomains file
    let mut subs_file = fs::File::create(format!("{}/subdomains.txt", args.path.clone())).expect("Error creating subdomains file");


    // Get subdomains
    let subs = get_subs(filtered_scopes);
    let subs_vec: Vec<&str> = subs.split("\n").collect();
    subs_file.write(subs.as_bytes()).expect("Error writing to subdomains file");
    println!("Found {} subdomains\n", subs_vec.len());

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
