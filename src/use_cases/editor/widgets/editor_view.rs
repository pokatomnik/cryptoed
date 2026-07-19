use cursive::{
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{NamedView, Panel, ResizedView, TextArea},
    wrap_impl,
};

pub(crate) struct EditorView {
    panel: ResizedView<Panel<ResizedView<NamedView<TextArea>>>>,
}

impl EditorView {
    pub fn new(initial_text: &str) -> Self {
        let textarea = TextArea::new()
            .content(initial_text)
            .with_name("editor")
            .full_screen();
        let panel = Panel::new(textarea).title("Editor").full_screen();
        Self { panel }
    }

    fn get_text(&mut self) -> String {
        self.panel
            .find_name::<TextArea>("editor")
            .map(|v| v.get_content().to_string())
            .unwrap_or_default()
    }

    fn set_text(&mut self, text: &str) {
        let textarea = self.panel.find_name::<TextArea>("editor");
        if let Some(mut textarea) = textarea {
            textarea.set_content(text);
        }
    }
}

impl ViewWrapper for EditorView {
    wrap_impl!(self.panel: ResizedView<Panel<ResizedView<NamedView<TextArea>>>>);
}
