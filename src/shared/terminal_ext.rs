use std::io::Stdout;

use cursive::backends::crossterm::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};

pub(crate) trait TerminalExt {
    fn set_selection(&self, enabled: bool) -> anyhow::Result<()>;
}

impl TerminalExt for Stdout {
    fn set_selection(&self, enabled: bool) -> anyhow::Result<()> {
        if enabled {
            execute!(std::io::stdout(), DisableMouseCapture)?;
        } else {
            execute!(std::io::stdout(), EnableMouseCapture)?;
        }
        Ok(())
    }
}
