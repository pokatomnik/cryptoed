use std::path::PathBuf;

use crate::{controllers::controller::Controller, use_cases::editor::app_ui::run_editor};
use clap::Args;

#[derive(Args)]
pub(crate) struct IndexController {
    /// Path to a file
    path: PathBuf,
}

impl Controller for IndexController {
    async fn handle(&self) -> anyhow::Result<()> {
        let path = self.path.to_string_lossy().to_string();
        let initial_text = std::fs::read_to_string(path.as_str()).unwrap_or_default();

        let save_path = path.clone();
        let result = run_editor(initial_text, Some(path.clone()), move |content| {
            std::fs::write(save_path.as_str(), content)?;
            Ok(())
        })?;

        if result.need_save() {
            std::fs::write(&path, result.content())?;
        }

        Ok(())
    }
}
