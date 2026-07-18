use std::path::PathBuf;

use crate::{controllers::controller::Controller, use_cases::editor::editor::run_editor};
use clap::Args;

#[derive(Args)]
pub(crate) struct IndexController {
    path: Option<PathBuf>,
}

impl Controller for IndexController {
    async fn handle(&self) -> anyhow::Result<()> {
        let result = run_editor("Hello".to_string());

        println!("{}", result);
        Ok(())
    }
}
