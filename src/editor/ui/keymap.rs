use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{action::Action, state::Focus};

#[derive(Debug, Clone, Copy)]
pub(crate) struct KeyHint {
    pub key: &'static str,
    pub description: &'static str,
}

pub(crate) const FOOTER_HINTS: &[KeyHint] = &[
    KeyHint {
        key: "Ctrl+S",
        description: "save and exit",
    },
    KeyHint {
        key: "Ctrl+X",
        description: "quit",
    },
    KeyHint {
        key: "Ctrl+C",
        description: "quit",
    },
    KeyHint {
        key: "Esc",
        description: "quit",
    },
];

pub(crate) fn action_for_key(key: KeyEvent, _focus: Focus) -> Option<Action> {
    if key.kind == KeyEventKind::Release {
        return None;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return match key.code {
            KeyCode::Char('s') => Some(Action::Save),
            KeyCode::Char('x') | KeyCode::Char('c') => Some(Action::Exit),
            _ => None,
        };
    }

    if key.code == KeyCode::Esc {
        return Some(Action::Exit);
    }

    None
}
