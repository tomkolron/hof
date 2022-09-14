mod args;
mod scopes;
mod bounties;

use std::env;

use args::get_args;
use bounties::get_bounties;
use scopes::get_scopes;

use stybulate::{Table, Style, Cell, Headers};

fn main() {
    // Set env var for more debugging info
    env::set_var("RUST_BACKTRACE", "1");

    // Get cli argumets
    let args = get_args();

    // Get domain scopes
    let scopes = get_scopes(args[0].clone());
    for scope in scopes.as_ref().unwrap().iter() {
        println!("{}", scope);
    }

    // Get bounties
    let bounties = get_bounties(args[0].clone());
    let bounty_table = Table::new(
        Style::Fancy,
        vec![
            vec![Cell::from("low"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[0]).as_str())],
            vec![Cell::from("medium"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[1]).as_str())],
            vec![Cell::from("high"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[2]).as_str())],
            vec![Cell::from("critical"), Cell::from(format!("{} USD", bounties.as_ref().unwrap()[3]).as_str())],
        ],
        Some(Headers::from(vec!["bounty", "prize"])),
    ).tabulate();
    println!("{}", bounty_table);
}
