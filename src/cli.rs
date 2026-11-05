mod positional;

use clap::Parser;


#[derive(Parser, Debug)]
pub struct Cli {
    pub query: String,
    /// note: does not use the `/search/english` endpoint and uses `/definition/english` instead
    #[arg(short, long)]
    pub entry_num: Option<u8>,
    #[arg(short, long)]
    pub pretty: bool,
}


