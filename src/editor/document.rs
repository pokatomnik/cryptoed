use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EditorShortcut {
    SaveAndExit,
    Exit,
}

#[derive(Debug, Clone)]
pub(crate) struct EditorDocument {
    path: Option<PathBuf>,
    text: String,
}

impl EditorDocument {
    pub(crate) fn new(path: Option<PathBuf>, text: String) -> Self {
        Self { path, text }
    }

    pub(crate) fn path_label(&self) -> String {
        self.path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "Untitled".to_string())
    }

    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub(crate) fn shortcut_for_key(&self, key: KeyEvent) -> Option<EditorShortcut> {
        if key.kind == KeyEventKind::Release {
            return None;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            return match key.code {
                KeyCode::Char('s') => Some(EditorShortcut::SaveAndExit),
                KeyCode::Char('x') | KeyCode::Char('c') => Some(EditorShortcut::Exit),
                _ => None,
            };
        }

        if key.code == KeyCode::Esc {
            return Some(EditorShortcut::Exit);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_events_are_ignored() {
        let document = EditorDocument::new(None, String::new());
        let key = KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Release,
            state: crossterm::event::KeyEventState::empty(),
        };

        assert_eq!(document.shortcut_for_key(key), None);
    }

    #[test]
    fn ctrl_s_triggers_save_and_exit() {
        let document = EditorDocument::new(None, String::new());
        let key = KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        };

        assert_eq!(
            document.shortcut_for_key(key),
            Some(EditorShortcut::SaveAndExit)
        );
    }
}
