use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::shared::terminal_ext::TerminalExt;
use crate::use_cases::editor::widgets::editor_view::EditorView;
use crate::use_cases::editor::widgets::shortcut_view::ShortcutView;

use cursive::With;
use cursive::direction::Orientation::Horizontal;
use cursive::direction::Orientation::Vertical;
use cursive::event::Event;
use cursive::event::Key::F2;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::TextArea;
use cursive::views::TextView;

pub(crate) fn run_editor(source: String) -> anyhow::Result<String> {
    let mut siv = cursive::default();
    let initial = source.clone();

    let selection_mode_enabled = Arc::new(AtomicBool::new(false));

    siv.load_toml(include_str!("./theme.toml"))
        .expect("Failed to load theme");

    let sme0 = selection_mode_enabled.clone();
    let sme1 = selection_mode_enabled.clone();
    siv.add_fullscreen_layer(
        LinearLayout::new(Vertical)
            .child(EditorView::new(initial.as_str()))
            .child(ShortcutView::new(sme0.load(Ordering::Relaxed)).with_name("selection_mode"))
            .full_screen(),
    );

    siv.add_global_callback(Event::CtrlChar('s'), |s| {
        let v = s.call_on_name("editor", |v: &mut TextArea| v.get_content().to_string());
        s.set_user_data(v.unwrap_or_default());
        s.quit();
    });

    siv.add_global_callback(Event::CtrlChar('c'), |_| {});

    siv.add_global_callback(Event::Key(F2), move |s| {
        let mut enabled = sme1.load(Ordering::Relaxed);
        enabled = !enabled;
        selection_mode_enabled.store(enabled, Ordering::Relaxed);
        s.call_on_name("selection_mode", |v: &mut ShortcutView| {
            v.set_selection_enabled(enabled)
        });
        _ = std::io::stdout().set_selection(enabled);
    });

    let mut runner = siv.into_runner();

    std::io::stdout().set_selection(false)?;

    runner.run();

    Ok("".to_string())
}

trait SelectionEnabled {
    fn selection_enabled_text(&self) -> String;
}

impl SelectionEnabled for bool {
    fn selection_enabled_text(&self) -> String {
        match self {
            true => "Selection mode (F2): enabled".to_string(),
            false => "Selection mode (F2): disabled".to_string(),
        }
    }
}
