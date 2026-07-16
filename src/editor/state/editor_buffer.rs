use crossterm::event::KeyEvent;
use ratatui_textarea::TextArea;

#[derive(Debug, Clone)]
pub(crate) struct EditorBuffer {
    textarea: TextArea<'static>,
    baseline: String,
    dirty: bool,
}

impl EditorBuffer {
    pub(crate) fn new(text: String) -> Self {
        let textarea = TextArea::new(text_to_lines(&text));
        Self {
            textarea,
            baseline: text,
            dirty: false,
        }
    }

    pub(crate) fn input(&mut self, key: KeyEvent) -> bool {
        self.textarea.input(key);
        self.dirty = self.text() != self.baseline;
        true
    }

    pub(crate) fn text(&self) -> String {
        self.textarea.lines().join("\n")
    }

    pub(crate) fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub(crate) fn content_length(&self) -> usize {
        self.textarea.lines().len().max(1)
    }

    pub(crate) fn screen_row(&self) -> usize {
        self.textarea.screen_cursor().row
    }

    pub(crate) fn textarea(&self) -> &TextArea<'static> {
        &self.textarea
    }
}

fn text_to_lines(text: &str) -> Vec<String> {
    text.split('\n').map(ToOwned::to_owned).collect()
}

#[cfg(test)]
mod tests {
    use super::EditorBuffer;

    #[test]
    fn text_round_trips_without_losing_trailing_newlines() {
        let samples = ["", "hello", "hello\n", "a\n\n", "α\nβ\n"];

        for sample in samples {
            let buffer = EditorBuffer::new(sample.to_string());
            assert_eq!(buffer.text(), sample);
        }
    }

    #[test]
    fn empty_buffer_is_clean() {
        let buffer = EditorBuffer::new(String::new());
        assert!(!buffer.is_dirty());
    }

    #[test]
    fn any_input_requests_redraw() {
        let mut buffer = EditorBuffer::new(String::from("ab"));
        let key = crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Left,
            modifiers: crossterm::event::KeyModifiers::empty(),
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        };

        assert!(buffer.input(key));
    }
}
