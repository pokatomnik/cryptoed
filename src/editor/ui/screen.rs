use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::editor::{
    action::Action,
    event::UiEvent,
    state::{EditorState, Focus},
    ui::{
        component::{Component, EventResult, UpdateResult},
        components::{footer::Footer, header::Header, text_editor::TextEditor},
        keymap,
    },
};

pub(crate) struct EditorScreen {
    header: Header,
    text_editor: TextEditor,
    footer: Footer,
}

impl EditorScreen {
    pub(crate) fn new() -> Self {
        Self {
            header: Header,
            text_editor: TextEditor::new("Editor"),
            footer: Footer,
        }
    }

    pub(crate) fn handle_event(
        &mut self,
        event: &UiEvent,
        state: &EditorState,
    ) -> EventResult<Action> {
        if let UiEvent::Key(key) = event {
            if let Some(action) = keymap::action_for_key(*key, state.focus()) {
                return EventResult::Action(action);
            }
        }

        match state.focus() {
            Focus::Editor => match self.text_editor.handle_event(event, state.buffer()) {
                EventResult::Action(editor_action) => {
                    EventResult::Action(Action::Editor(editor_action))
                }
                EventResult::Consumed => EventResult::Consumed,
                EventResult::Ignored => EventResult::Ignored,
            },
        }
    }

    pub(crate) fn update(
        &mut self,
        action: &Action,
        state: &mut EditorState,
    ) -> UpdateResult<Action> {
        match action {
            Action::Editor(editor_action) => {
                let result = self.text_editor.update(editor_action, state.buffer_mut());
                UpdateResult {
                    redraw: result.redraw,
                    follow_up: result.follow_up.into_iter().map(Action::Editor).collect(),
                }
            }
            Action::Save | Action::Exit => UpdateResult::none(),
        }
    }

    pub(crate) fn render(&self, frame: &mut Frame<'_>, state: &EditorState) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .split(frame.area());

        self.header.render(frame, chunks[0], &Header::view(state));
        self.text_editor.render(frame, chunks[1], state.buffer());
        self.footer.render(frame, chunks[2], &Footer::view(state));
    }
}
