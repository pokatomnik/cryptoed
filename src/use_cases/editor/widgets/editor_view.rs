use std::sync::Arc;

use cursive::{
    Cursive, View,
    event::{Event, EventResult},
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{NamedView, Panel, ResizedView, TextArea},
    wrap_impl,
};

use crate::use_cases::editor::shared::consts::VIEW_NOT_FOUND;

type OnChange = Arc<dyn Fn(&mut Cursive, &str) + Send + Sync>;

pub(crate) struct EditorView {
    panel: ResizedView<NamedView<Panel<ResizedView<NamedView<TextArea>>>>>,
    on_change: OnChange,
    title: String,
    modified: bool,
}

impl EditorView {
    fn get_default_title() -> &'static str {
        "Editor"
    }

    pub fn shorten<S: Into<String>>(value: S, limit: usize, edge: usize) -> String {
        let value = value.into();
        let chars: Vec<char> = value.chars().collect();

        if chars.len() <= limit || chars.len() <= edge.saturating_mul(2) + 3 {
            return value;
        }

        let prefix: String = chars.iter().take(edge).copied().collect();
        let suffix: String = chars
            .iter()
            .rev()
            .take(edge)
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        format!("{prefix}...{suffix}")
    }

    pub fn new(initial_text: &str, on_change: OnChange, title: Option<impl Into<String>>) -> Self {
        let title = title
            .map(|v| v.into())
            .map(|v| Self::shorten(v, 50, 10))
            .unwrap_or_else(|| Self::get_default_title().to_string());
        let textarea = TextArea::new()
            .content(initial_text)
            .with_name("editor")
            .full_screen();
        let panel = Panel::new(textarea)
            .title(title.clone())
            .with_name("panel")
            .full_screen();
        Self {
            panel,
            on_change,
            title: title,
            modified: false,
        }
    }

    pub fn get_modified(&mut self) -> bool {
        self.modified
    }

    pub fn set_modified(&mut self, modified: bool) {
        let title = self.title.clone();
        self.modified = modified;
        let mut panel = self
            .panel
            .find_name::<Panel<ResizedView<NamedView<TextArea>>>>("panel")
            .expect(VIEW_NOT_FOUND);
        if modified {
            panel.set_title(format!("* {title}"));
        } else {
            panel.set_title(title);
        }
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        let title = title.into();
        let mut panel = self
            .panel
            .find_name::<Panel<ResizedView<NamedView<TextArea>>>>("panel")
            .expect(VIEW_NOT_FOUND);
        self.title = title.clone();
        let mut display_title = String::from(title);
        if self.modified {
            display_title = format!("* {display_title}");
        }
        panel.set_title(display_title);
    }

    #[allow(unused)]
    pub fn get_text(&mut self) -> String {
        self.panel
            .find_name::<TextArea>("editor")
            .map(|v| v.get_content().to_string())
            .unwrap_or_default()
    }

    #[allow(unused)]
    pub fn set_text(&mut self, text: &str) {
        let mut textarea = self
            .panel
            .find_name::<TextArea>("editor")
            .expect(VIEW_NOT_FOUND);
        textarea.set_content(text);
    }
}

impl ViewWrapper for EditorView {
    wrap_impl!(self.panel: ResizedView<NamedView<Panel<ResizedView<NamedView<TextArea>>>>>);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        let mut textarea = self
            .panel
            .find_name::<TextArea>("editor")
            .expect(VIEW_NOT_FOUND);
        let old_content = textarea.get_content().to_string();

        let result = textarea.on_event(event);

        if textarea.get_content() == old_content {
            return result;
        }

        let new_content = textarea.get_content().to_owned();
        let on_change = self.on_change.clone();

        result.and(EventResult::with_cb(move |siv| {
            on_change(siv, &new_content);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::EditorView;

    #[test]
    fn shorten_keeps_short_strings_intact() {
        assert_eq!(EditorView::shorten("hello", 10, 3), "hello");
    }

    #[test]
    fn shorten_truncates_long_strings_from_both_ends() {
        assert_eq!(EditorView::shorten("abcdefghijk", 8, 3), "abc...ijk");
    }

    #[test]
    fn shorten_handles_unicode_characters() {
        assert_eq!(EditorView::shorten("абвгдежзий", 6, 2), "аб...ий");
    }
}
