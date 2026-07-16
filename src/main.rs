use clap::Parser;

use crate::{cmd::cli, controllers::controller::Controller};

mod cmd;
mod controllers;
mod editor;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::try_parse();
    let Ok(cli) = cli else {
        eprintln!("Failed to parse commandline arguments");
        return;
    };

    let result = cli.index.handle().await;

    if let Err(e) = result {
        eprintln!("Failed to run editor: {e}");
    }
}
