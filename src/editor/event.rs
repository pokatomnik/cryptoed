use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum UiEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Resize { width: u16, height: u16 },
    FocusGained,
    FocusLost,
    Tick,
}

impl UiEvent {
    pub(crate) fn from_crossterm(event: CrosstermEvent) -> Option<Self> {
        match event {
            CrosstermEvent::Key(key) => Some(Self::Key(key)),
            CrosstermEvent::Mouse(mouse) => Some(Self::Mouse(mouse)),
            CrosstermEvent::Paste(text) => Some(Self::Paste(text)),
            CrosstermEvent::Resize(width, height) => Some(Self::Resize { width, height }),
            CrosstermEvent::FocusGained => Some(Self::FocusGained),
            CrosstermEvent::FocusLost => Some(Self::FocusLost),
        }
    }
}
