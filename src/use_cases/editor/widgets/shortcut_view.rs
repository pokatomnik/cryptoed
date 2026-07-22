use cursive::{
    direction::Orientation,
    view::{Finder, Nameable, ViewWrapper},
    views::{LinearLayout, TextView},
    wrap_impl,
};

use crate::use_cases::editor::shared::consts::VIEW_NOT_FOUND;

pub(crate) struct ShortcutView {
    layout: LinearLayout,
    selection_enabled: bool,
    preview_enabled: bool,
}

impl ShortcutView {
    fn get_selection_enabled_text(selection_enabled: bool) -> &'static str {
        match selection_enabled {
            true => "Selection (F2): enabled",
            false => "Selection (F2): disabled",
        }
    }

    fn get_preview_enabled_text(preview_enabled: bool) -> &'static str {
        match preview_enabled {
            true => "Mode (Ctrl+R): preview",
            false => "Mode (Ctrl+R): edit",
        }
    }

    pub fn new(selection_enabled: bool, preview_enabled: bool) -> Self {
        let mut layout = LinearLayout::new(Orientation::Horizontal);

        let selection_enabled_text = Self::get_selection_enabled_text(selection_enabled);
        let preview_enabled_text = Self::get_preview_enabled_text(preview_enabled);

        layout.add_child(TextView::new(selection_enabled_text).with_name("selection_enabled"));
        layout.add_child(TextView::new(" | "));
        layout.add_child(TextView::new(preview_enabled_text).with_name("preview_enabled"));
        layout.add_child(TextView::new(" | "));
        layout.add_child(TextView::new("Save (Ctrl+S)"));
        layout.add_child(TextView::new(" | "));
        layout.add_child(TextView::new("Exit (Ctrl+X)"));

        Self {
            selection_enabled,
            preview_enabled,
            layout,
        }
    }

    pub fn set_selection_enabled(&mut self, selection_enabled: bool) {
        self.selection_enabled = selection_enabled;
        let mut text_view = self
            .layout
            .find_name::<TextView>("selection_enabled")
            .expect(VIEW_NOT_FOUND);
        text_view.set_content(Self::get_selection_enabled_text(selection_enabled));
    }

    #[allow(unused)]
    pub fn get_selection_enabled(&self) -> bool {
        self.selection_enabled
    }

    pub fn set_preview_enabled(&mut self, preview_enabled: bool) {
        self.preview_enabled = preview_enabled;
        let mut text_view = self
            .layout
            .find_name::<TextView>("preview_enabled")
            .expect(VIEW_NOT_FOUND);
        text_view.set_content(Self::get_preview_enabled_text(preview_enabled));
    }

    #[cfg(test)]
    pub fn get_preview_enabled(&self) -> bool {
        self.preview_enabled
    }
}

impl ViewWrapper for ShortcutView {
    wrap_impl!(self.layout: LinearLayout);
}
