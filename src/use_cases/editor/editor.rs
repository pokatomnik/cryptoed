use std::{
    alloc::Layout,
    ops::Deref,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use cursive::{
    Cursive, View, With,
    direction::Orientation::{Horizontal, Vertical},
    event::{Event, EventTrigger},
    theme::Theme,
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, LinearLayout, Panel, TextArea, TextView},
};

use crate::use_cases::editor::theme::get_theme;

pub(crate) fn run_editor(source: String) -> String {
    let mut siv = cursive::default();
    let initial = source.clone();

    siv.set_theme(get_theme());
    siv.set_user_data(source.clone());

    siv.add_fullscreen_layer(
        LinearLayout::new(Vertical)
            .child(
                Panel::new(
                    TextArea::new()
                        .with(move |v| {
                            v.set_content(initial);
                        })
                        .with_name("editor")
                        .full_screen(),
                )
                .title("Editor")
                .full_height(),
            )
            .full_screen(),
    );

    siv.add_global_callback(Event::CtrlChar('s'), |s| {
        let v = s.call_on_name("editor", |v: &mut TextArea| v.get_content().to_string());
        s.set_user_data(v.unwrap_or_default());
        s.quit();
    });

    // Starts the event loop.
    siv.run();

    let result = siv
        .user_data::<String>()
        .map(|v| v.to_string())
        .unwrap_or_default();

    result
}
