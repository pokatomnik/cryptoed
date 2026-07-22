use cursive::{
    utils::markup::markdown,
    view::{Resizable, Scrollable, ViewWrapper},
    views::{Panel, ResizedView, ScrollView, TextView},
    wrap_impl,
};

pub(crate) struct PreviewView {
    panel: ResizedView<Panel<ScrollView<TextView>>>,
}

impl PreviewView {
    pub fn new(text: impl Into<String>) -> Self {
        let content = TextView::new(markdown::parse(text)).scrollable();
        let panel = Panel::new(content).title("Preview").full_screen();

        Self { panel }
    }
}

impl ViewWrapper for PreviewView {
    wrap_impl!(self.panel: ResizedView<Panel<ScrollView<TextView>>>);
}
