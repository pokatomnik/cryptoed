use std::path::PathBuf;

use crate::{
    controllers::controller::Controller,
    editor::{editor::run_editor, editor_config::EditorConfig},
};
use clap::Args;

#[derive(Args)]
pub(crate) struct IndexController {
    path: Option<PathBuf>,
}

impl Controller for IndexController {
    async fn handle(&self) -> anyhow::Result<()> {
        let editor_config = EditorConfig::new().with_path(self.path.as_ref());
        let result = run_editor(editor_config).await?;

        match result.reason() {
            crate::editor::outcome::FinishReason::SaveAndExit
            | crate::editor::outcome::FinishReason::Exit => {
                println!("{}", result.text());
            }
        }
        Ok(())
    }
}
