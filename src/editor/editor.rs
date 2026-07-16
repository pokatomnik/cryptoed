use std::io;

use anyhow::Context;
use crossterm::{
    event::{Event, EventStream, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures_util::StreamExt;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::editor::{
    document::{EditorDocument, EditorShortcut},
    editor_config::EditorConfig,
    ui::{text_editor::TextEditor, ui},
};

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalSession {
    fn enter() -> anyhow::Result<Self> {
        enable_raw_mode().context("failed to enable raw mode")?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).context("failed to create terminal")?;

        Ok(Self { terminal })
    }

    fn draw<F>(&mut self, render: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut ratatui::Frame<'_>),
    {
        self.terminal.draw(render).context("failed to draw frame")?;
        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = self.terminal.show_cursor();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

pub(crate) async fn run_editor(config: EditorConfig) -> anyhow::Result<String> {
    let path = config.path().map(|path| path.to_path_buf());
    let initial_text = match config.path() {
        Some(path) => tokio::fs::read_to_string(path).await.unwrap_or_default(),
        None => String::new(),
    };

    let document = EditorDocument::new(path, initial_text.clone());
    let editor = TextEditor::new(&initial_text);

    run_event_loop(document, editor).await
}

async fn run_event_loop(
    mut document: EditorDocument,
    mut editor: TextEditor,
) -> anyhow::Result<String> {
    let mut terminal = TerminalSession::enter()?;
    let mut events = EventStream::new();

    terminal.draw(|frame| ui::draw(frame, &document, &editor))?;

    while let Some(event) = events.next().await {
        let event = event.context("failed to read terminal event")?;

        match event {
            Event::Key(key) if key.kind != KeyEventKind::Release => {
                if let Some(shortcut) = document.shortcut_for_key(key) {
                    match shortcut {
                        EditorShortcut::SaveAndExit | EditorShortcut::Exit => {
                            return Ok(document.text().to_string());
                        }
                    }
                } else {
                    editor.input(key);
                    document.set_text(editor.text());
                }
            }
            Event::Resize(_, _) => {}
            _ => {}
        }

        terminal.draw(|frame| ui::draw(frame, &document, &editor))?;
    }

    Ok(document.text().to_string())
}
