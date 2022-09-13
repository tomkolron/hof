mod args;
mod scopes;
mod bounties;

use std::env;

use args::get_args;
use bounties::get_bounties;
use scopes::get_scopes;

use stybulate::{Table, Style, Cell, Headers};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args = get_args();
    println!("{:?}", args);
    let scopes = get_scopes(args[0].clone());
    for scope in scopes.as_ref().unwrap().iter() {
        println!("{}", scope);
    }
    let bounties = get_bounties(args[0].clone());
    println!("{:?}", bounties);
    // for scope in bounties.as_ref().unwrap().iter() {
    //     println!("{}", scope);
    // }
    let bounty = Table::new(
        Style::Fancy,
        vec![
            vec![Cell::from("low"), Cell::from(format!("{} USD", 100).as_str())],
            vec![Cell::from("medium"), Cell::from(format!("{} USD", 500).as_str())],
            vec![Cell::from("high"), Cell::from(format!("{} USD", 1000).as_str())],
            vec![Cell::from("critical"), Cell::from(format!("{} USD", 5000).as_str())],
        ],
        Some(Headers::from(vec!["bounty", "prize"])),
    ).tabulate();
    println!("{}", bounty);
}
