use clap::Parser;

use crate::{cmd::cli, controllers::controller::Controller};

mod cmd;
mod controllers;
mod shared;
mod use_cases;

fn main() {
    let cli = cli::Cli::parse();

    let result = cli.index.handle();

    if let Err(e) = result {
        eprintln!("Failed to run editor: {e}");
    }
}
