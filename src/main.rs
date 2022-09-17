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
    println!("Creating project directory ...");
    let dir = fs::create_dir(args.path.clone());
    match dir {
        Ok(()) => {},
        Err(error) => match error.kind() {
            io::ErrorKind::AlreadyExists => overwrite_directory(args.path.clone()),
            other_error => panic!("Problem creating directory: {:?}", other_error),
        }
    }


    // Create scopes file
    let scopes_file = fs::File::create(format!("{}/scopes.txt", args.path.clone()));
    let mut scopes_file = match scopes_file {
        Ok(file) => file,
        Err(error) => panic!("Error creating scopes file: {:?}", error.kind()),
    };

    // Get domain scopes
    println!("Writing to scopes file:");
    let scopes = get_scopes(args.query.clone());
    for scope in scopes.as_ref().unwrap().iter() {
        println!("{}", scope);
        let file_scope = format!("{}\n", scope);
        scopes_file.write(file_scope.as_bytes()).unwrap();
    }

    // Get subdomains
    let subs = get_subs(scopes.as_ref().unwrap().to_vec());
    let subs_vec: Vec<&str> = subs.split("\n").collect();
    println!("Found {} subdomains", subs_vec.len());

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
    let read_line = stdin.read_line(&mut user_input);
    match read_line {
        Ok(_a) => {},
        Err(error) => panic!("Problem reading user input: {:?}", error.kind()),
    }
    if user_input == "y\n" || user_input == "Y\n" {
        println!("overwriting directory ...");
        let del_dir = fs::remove_dir_all(&path);
        match del_dir {
            Ok(()) => {},
            Err(error) => panic!("There was a problem overwriting directory: {:?}", error.kind()),
        }
        let re_dir = fs::create_dir(&path);
        match re_dir {
            Ok(()) => {},
            Err(error) => panic!("There was a problem overwriting directory: {:?}", error.kind()),
        }
    }else {
        quit()
    }
}

fn quit() {
    println!("GoodBye \u{1f44b}");
    process::abort();
}
