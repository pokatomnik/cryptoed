use std::sync::Arc;

use cursive::{
    Cursive,
    direction::Orientation::Vertical,
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{LinearLayout, ResizedView},
    wrap_impl,
};

use crate::use_cases::editor::{
    shared::consts::VIEW_NOT_FOUND,
    widgets::{editor_view::EditorView, shortcut_view::ShortcutView},
};

pub(crate) struct EditorLayer {
    layer: ResizedView<LinearLayout>,
}

impl EditorLayer {
    pub fn new<F>(
        text_to_edit: &str,
        title: Option<impl Into<String>>,
        selection_enabled: bool,
        on_change: F,
    ) -> Self
    where
        F: Fn((&mut Cursive, bool)) + Send + Sync + 'static,
    {
        let title = title.map(|v| v.into());
        let on_change = Arc::new(on_change);
        let initial_text = text_to_edit.to_owned();

        let on_change_clone = on_change.clone();
        let layout = LinearLayout::new(Vertical)
            .child(
                EditorView::new(
                    text_to_edit,
                    Arc::new(move |s, txt| on_change_clone((s, initial_text == txt))),
                    title,
                )
                .with_name("editor_view"),
            )
            .child(ShortcutView::new(selection_enabled).with_name("selection_mode"))
            .full_screen();
        Self { layer: layout }
    }

    pub fn set_modified(&mut self, modified: bool) {
        let mut editor_view = self
            .layer
            .find_name::<EditorView>("editor_view")
            .expect(VIEW_NOT_FOUND);
        editor_view.set_modified(modified);
    }

    pub fn get_modified(&mut self) -> bool {
        let mut editor_view = self
            .layer
            .find_name::<EditorView>("editor_view")
            .expect(VIEW_NOT_FOUND);
        editor_view.get_modified()
    }

    #[allow(unused)]
    pub fn set_title(&mut self, title: impl Into<String>) {
        let mut editor_view = self
            .layer
            .find_name::<EditorView>("editor_view")
            .expect(VIEW_NOT_FOUND);
        editor_view.set_title(title);
    }

    pub fn toggle_selection_mode(&mut self) {
        let mut selection_mode = self
            .layer
            .find_name::<ShortcutView>("selection_mode")
            .expect(VIEW_NOT_FOUND);
        let enabled = !selection_mode.get_selection_enabled();
        selection_mode.set_selection_enabled(enabled);
    }

    pub fn get_selection_mode(&mut self) -> bool {
        let selection_mode = self
            .layer
            .find_name::<ShortcutView>("selection_mode")
            .expect(VIEW_NOT_FOUND);
        selection_mode.get_selection_enabled()
    }

    pub fn get_content(&mut self) -> String {
        let mut editor_view = self
            .layer
            .find_name::<EditorView>("editor_view")
            .expect(VIEW_NOT_FOUND);
        editor_view.get_text()
    }
}

impl ViewWrapper for EditorLayer {
    wrap_impl!(self.layer: ResizedView<LinearLayout>);
}
