use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::editor::{
    state::EditorState,
    ui::component::{Component, EventResult, UpdateResult},
};

#[derive(Debug, Clone)]
pub(crate) struct HeaderView {
    pub title: String,
    pub dirty: bool,
}

#[derive(Debug, Default)]
pub(crate) struct Header;

impl Header {
    pub(crate) fn view(state: &EditorState) -> HeaderView {
        HeaderView {
            title: state.document().path_label(),
            dirty: state.buffer().is_dirty(),
        }
    }
}

impl Component for Header {
    type State = HeaderView;
    type Action = ();

    fn handle_event(
        &mut self,
        _event: &crate::editor::event::UiEvent,
        _state: &Self::State,
    ) -> EventResult<Self::Action> {
        EventResult::Ignored
    }

    fn update(
        &mut self,
        _action: &Self::Action,
        _state: &mut Self::State,
    ) -> UpdateResult<Self::Action> {
        UpdateResult::none()
    }

    fn render(&self, frame: &mut Frame<'_>, area: Rect, state: &Self::State) {
        let dirty = if state.dirty { " *" } else { "" };
        let title = format!("Editing: {}{}", state.title, dirty);

        let paragraph = Paragraph::new(Line::from(vec![
            Span::styled(
                "cryptoed",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" — "),
            Span::raw(title),
        ]))
        .block(Block::bordered().borders(Borders::ALL))
        .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
