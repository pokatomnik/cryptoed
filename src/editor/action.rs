use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub(crate) enum Action {
    Editor(EditorAction),
    Save,
    Exit,
}

#[derive(Debug, Clone)]
pub(crate) enum EditorAction {
    Input(KeyEvent),
}
