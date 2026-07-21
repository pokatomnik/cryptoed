use clap::Parser;

use crate::{cmd::cli, controllers::controller::Controller};

mod cmd;
mod controllers;
mod shared;
mod use_cases;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    let result = cli.index.handle().await;

    if let Err(e) = result {
        eprintln!("Failed to run editor: {e}");
    }
}
