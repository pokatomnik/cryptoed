use std::sync::Arc;

use cursive::{
    Cursive,
    direction::Orientation::Vertical,
    view::{Finder, Nameable, Resizable, ViewWrapper},
    views::{LinearLayout, ResizedView, StackView},
    wrap_impl,
};

use crate::use_cases::editor::{
    shared::consts::VIEW_NOT_FOUND,
    widgets::{editor_view::EditorView, preview_view::PreviewView, shortcut_view::ShortcutView},
};

pub(crate) struct EditorLayer {
    layer: ResizedView<LinearLayout>,
    preview_enabled: bool,
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
        let mut content = StackView::new();
        content.add_fullscreen_layer(
            EditorView::new(
                text_to_edit,
                Arc::new(move |s, txt| on_change_clone((s, initial_text == txt))),
                title,
            )
            .with_name("editor_view"),
        );

        let layout = LinearLayout::new(Vertical)
            .child(content.with_name("content_view").full_screen())
            .child(ShortcutView::new(selection_enabled, false).with_name("shortcut_view"))
            .full_screen();
        Self {
            layer: layout,
            preview_enabled: false,
        }
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
            .find_name::<ShortcutView>("shortcut_view")
            .expect(VIEW_NOT_FOUND);
        let enabled = !selection_mode.get_selection_enabled();
        selection_mode.set_selection_enabled(enabled);
    }

    pub fn get_selection_mode(&mut self) -> bool {
        let selection_mode = self
            .layer
            .find_name::<ShortcutView>("shortcut_view")
            .expect(VIEW_NOT_FOUND);
        selection_mode.get_selection_enabled()
    }

    pub fn toggle_preview_mode(&mut self) {
        if self.preview_enabled {
            self.layer
                .find_name::<StackView>("content_view")
                .expect(VIEW_NOT_FOUND)
                .pop_layer()
                .expect(VIEW_NOT_FOUND);
        } else {
            let content = self.get_content();
            self.layer
                .find_name::<StackView>("content_view")
                .expect(VIEW_NOT_FOUND)
                .add_fullscreen_layer(PreviewView::new(content).with_name("preview_view"));
        }

        self.preview_enabled = !self.preview_enabled;
        self.layer
            .find_name::<ShortcutView>("shortcut_view")
            .expect(VIEW_NOT_FOUND)
            .set_preview_enabled(self.preview_enabled);
    }

    #[cfg(test)]
    pub fn get_preview_mode(&self) -> bool {
        self.preview_enabled
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

#[cfg(test)]
mod tests {
    use cursive::{View, event::Event, view::Finder};

    use super::EditorLayer;
    use crate::use_cases::editor::widgets::shortcut_view::ShortcutView;

    #[test]
    fn preview_mode_does_not_edit_content() {
        let mut layer = EditorLayer::new("initial", None::<String>, false, |_| {});

        layer.toggle_preview_mode();
        assert!(layer.get_preview_mode());
        assert!(
            layer
                .layer
                .find_name::<ShortcutView>("shortcut_view")
                .expect("shortcut view should exist")
                .get_preview_enabled()
        );

        _ = layer.on_event(Event::Char('x'));
        assert_eq!(layer.get_content(), "initial");

        layer.toggle_preview_mode();
        assert!(!layer.get_preview_mode());
        assert_eq!(layer.get_content(), "initial");
    }
}
