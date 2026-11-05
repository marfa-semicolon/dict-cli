use clap::Parser;

use crate::cli::Cli;

mod cli;
mod command;
mod client;
mod dict;
mod parser;
mod utils;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    command::run(cli).await;
}
