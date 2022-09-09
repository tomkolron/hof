mod args;
mod request;

use args::get_args;
use request::get_scopes;

fn main() {
    let args = get_args();
    println!("{:?}", args);
    let scopes = get_scopes(args[0].clone());
    for scope in scopes.iter() {
        println!("{}", scope);
    }
}
