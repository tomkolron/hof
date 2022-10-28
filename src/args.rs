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
}
