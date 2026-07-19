use cursive::{
    direction::Orientation,
    view::{Finder, Nameable, ViewWrapper},
    views::{LinearLayout, TextView},
    wrap_impl,
};

pub(crate) struct ShortcutView {
    layout: LinearLayout,
    selection_enabled: bool,
}

impl ShortcutView {
    fn get_selection_enabled_text(selection_enabled: bool) -> &'static str {
        match selection_enabled {
            true => "Selection (F2): enabled",
            false => "Selection (F2): disabled",
        }
    }

    pub fn new(selection_enabled: bool) -> Self {
        let mut layout = LinearLayout::new(Orientation::Horizontal);

        let selection_enabled_text = Self::get_selection_enabled_text(selection_enabled);

        layout.add_child(TextView::new(selection_enabled_text).with_name("selection_enabled"));
        layout.add_child(TextView::new(" | "));
        layout.add_child(TextView::new("Save (Ctrl+S)"));

        Self {
            selection_enabled,
            layout: layout,
        }
    }

    pub fn set_selection_enabled(&mut self, selection_enabled: bool) {
        self.selection_enabled = selection_enabled;
        let text_view = self.layout.find_name::<TextView>("selection_enabled");
        if let Some(mut text_view) = text_view {
            text_view.set_content(Self::get_selection_enabled_text(selection_enabled));
        };
    }

    #[allow(unused)]
    pub fn get_selection_enabled(&self) -> bool {
        self.selection_enabled
    }
}

impl ViewWrapper for ShortcutView {
    wrap_impl!(self.layout: LinearLayout);
}
