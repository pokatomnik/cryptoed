use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::editor::{
    state::{EditorState, Focus},
    ui::{
        component::{Component, EventResult, UpdateResult},
        keymap::{FOOTER_HINTS, KeyHint},
    },
};

#[derive(Debug, Clone)]
pub(crate) struct FooterView {
    pub focus_label: String,
    pub hints: Vec<KeyHint>,
}

#[derive(Debug, Default)]
pub(crate) struct Footer;

impl Footer {
    pub(crate) fn view(state: &EditorState) -> FooterView {
        let focus_label = match state.focus() {
            Focus::Editor => "Editor".to_string(),
        };

        FooterView {
            focus_label,
            hints: FOOTER_HINTS.to_vec(),
        }
    }
}

impl Component for Footer {
    type State = FooterView;
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
        let mut spans = vec![Span::styled(
            format!("Focus: {}  ", state.focus_label),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )];

        for hint in &state.hints {
            spans.push(Span::styled(
                hint.key,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw(format!(" {}  ", hint.description)));
        }

        frame.render_widget(
            Paragraph::new(Line::from(spans)).style(Style::default().fg(Color::DarkGray)),
            area,
        );
    }
}
