use cursive::{
    view::{Resizable, Scrollable, ViewWrapper},
    views::{Panel, ResizedView, ScrollView, TextView},
    wrap_impl,
};

use crate::shared::markdown::render_markdown;

pub(crate) struct PreviewView {
    panel: ResizedView<Panel<ScrollView<TextView>>>,
}

impl PreviewView {
    pub fn new(text: impl Into<String>) -> Self {
        let content = TextView::new(render_markdown(&text.into())).scrollable();
        let panel = Panel::new(content).title("Preview").full_screen();

        Self { panel }
    }
}

impl ViewWrapper for PreviewView {
    wrap_impl!(self.panel: ResizedView<Panel<ScrollView<TextView>>>);
}
