use clap::Parser;

use crate::controllers::index::IndexController;

#[derive(Parser)]
#[command(name = "cryptoed")]
#[command(about = "Simple TUI editor for encrypted files")]
#[command(version)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub index: IndexController,
}
