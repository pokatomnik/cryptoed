use std::path::PathBuf;

use crate::{controllers::controller::Controller, use_cases::editor::app_ui::run_editor};
use clap::Args;

#[derive(Args)]
pub(crate) struct IndexController {
    path: Option<PathBuf>,
}

impl Controller for IndexController {
    async fn handle(&self) -> anyhow::Result<()> {
        let result = run_editor("Hello".to_string(), None::<String>)?;

        println!("content: \"{}\"", result.content());
        println!("need_save: {}", result.need_save());
        Ok(())
    }
}
