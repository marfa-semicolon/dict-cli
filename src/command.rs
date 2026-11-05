use std::{io::Write, process::Stdio};

use serde::Serialize;

use crate::{cli::Cli, client::fetch};

pub async fn run(cli: Cli) {
    let query = cli.query;
    let entry_num = cli.entry_num;
    let pretty = cli.pretty;

    let entry = fetch(query, entry_num.unwrap_or(1)).await;
    let output = if pretty {
        println!("json pretty");
        serde_json::to_string_pretty(&entry).unwrap()
    } else {
        serde_json::to_string(&entry).unwrap()
    };

    std::fs::write("out.json", output).unwrap();

    // println!("{}", output);
}
