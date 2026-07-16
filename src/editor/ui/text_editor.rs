use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use ratatui_textarea::TextArea;

pub(crate) struct TextEditor {
    textarea: TextArea<'static>,
}

impl TextEditor {
    pub(crate) fn new(initial_text: &str) -> Self {
        Self {
            textarea: TextArea::new(text_to_lines(initial_text)),
        }
    }

    pub(crate) fn input(&mut self, key: KeyEvent) {
        self.textarea.input(key);
    }

    pub(crate) fn text(&self) -> String {
        self.textarea.lines().join("\n")
    }

    pub(crate) fn render(&self, frame: &mut Frame<'_>, area: Rect, title: &str) {
        let block = Block::bordered().title(title);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let content_length = self.textarea.lines().len().max(1);
        let viewport_height = inner.height as usize;
        let needs_scrollbar = content_length > viewport_height && inner.width > 1;

        let (editor_area, scrollbar_area) = if needs_scrollbar {
            let chunks =
                Layout::horizontal([Constraint::Min(1), Constraint::Length(1)]).split(inner);
            (chunks[0], Some(chunks[1]))
        } else {
            (inner, None)
        };

        frame.render_widget(&self.textarea, editor_area);

        if let Some(scrollbar_area) = scrollbar_area {
            let position = self
                .textarea
                .screen_cursor()
                .row
                .min(content_length.saturating_sub(1));
            let mut scrollbar_state = ScrollbarState::new(content_length)
                .position(position)
                .viewport_content_length(viewport_height);

            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                scrollbar_area,
                &mut scrollbar_state,
            );
        }
    }
}

fn text_to_lines(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }

    let mut lines: Vec<String> = text.split('\n').map(ToOwned::to_owned).collect();
    if text.ends_with('\n') {
        lines.push(String::new());
    }

    lines
}
