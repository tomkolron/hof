use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct FbbArgs {
    /// A query for hackerone
    #[clap(short, long, value_parser)]
    pub query: String,
    /// A path you want your project to be saved at
    #[clap(short, long, value_parser)]
    pub path: String,
    /// Set timeout for each request in seconds
    #[clap(short, long, value_parser, default_value_t = 8)]
    pub timeout: u64,
    /// Tell the program wether to find subdomains for scopes with wildcards or not
    #[clap(short, long, value_parser, default_value = "true")]
    pub subdomains: String,
    /// Tell the program wether to get http headers for all domains or not
    #[clap(short = 'H', long, value_parser, default_value = "true")]
    pub headers: String,
}
