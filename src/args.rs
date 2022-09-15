use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct FbbArgs {
    /// a query for hackerone
    #[clap(short, long, value_parser)]
    pub query: String,
    /// a path you want your project to be saved at
    #[clap(short, long, value_parser)]
    pub path: String,
}
