use crossterm::event::KeyEventKind;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::editor::{
    action::EditorAction,
    state::EditorBuffer,
    ui::component::{Component, EventResult, UpdateResult},
};

#[derive(Debug)]
pub(crate) struct TextEditor {
    title: String,
}

impl TextEditor {
    pub(crate) fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    fn should_show_scrollbar(area: Rect, buffer: &EditorBuffer) -> bool {
        buffer.content_length() > area.height as usize && area.width > 1
    }

    fn render_scrollbar(&self, frame: &mut Frame<'_>, area: Rect, buffer: &EditorBuffer) {
        let content_length = buffer.content_length();
        let position = buffer.screen_row().min(content_length.saturating_sub(1));
        let mut state = ScrollbarState::new(content_length)
            .position(position)
            .viewport_content_length(area.height as usize);

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut state,
        );
    }
}

impl Component for TextEditor {
    type State = EditorBuffer;
    type Action = EditorAction;

    fn handle_event(
        &mut self,
        event: &crate::editor::event::UiEvent,
        _state: &Self::State,
    ) -> EventResult<Self::Action> {
        match event {
            crate::editor::event::UiEvent::Key(key) if key.kind != KeyEventKind::Release => {
                EventResult::Action(EditorAction::Input(*key))
            }
            _ => EventResult::Ignored,
        }
    }

    fn update(
        &mut self,
        action: &Self::Action,
        state: &mut Self::State,
    ) -> UpdateResult<Self::Action> {
        match action {
            EditorAction::Input(key) => {
                if state.input(*key) {
                    UpdateResult::redraw()
                } else {
                    UpdateResult::none()
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame<'_>, area: Rect, state: &Self::State) {
        let block = Block::bordered().title(self.title.as_str());
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let needs_scrollbar = Self::should_show_scrollbar(inner, state);
        let (editor_area, scrollbar_area) = if needs_scrollbar {
            let chunks =
                Layout::horizontal([Constraint::Min(1), Constraint::Length(1)]).split(inner);
            (chunks[0], Some(chunks[1]))
        } else {
            (inner, None)
        };

        frame.render_widget(state.textarea(), editor_area);

        if let Some(scrollbar_area) = scrollbar_area {
            self.render_scrollbar(frame, scrollbar_area, state);
        }
    }
}
