use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::shared::terminal_ext::TerminalExt;
use crate::use_cases::editor::editor_run_result::EditorRunResult;
use crate::use_cases::editor::layers::confirm_save_layer::ConfirmSaveLayer;
use crate::use_cases::editor::layers::editor_layer::EditorLayer;
use crate::use_cases::editor::layers::error_layer::ErrorLayer;
use crate::use_cases::editor::shared::consts::VIEW_NOT_FOUND;

use cursive::event::Event;
use cursive::event::Key::F2;
use cursive::view::Nameable;

pub(crate) fn run_editor(
    text_to_edit: String,
    source: Option<impl Into<String>>,
    on_save: impl Fn(String) -> anyhow::Result<()> + Send + Sync + 'static,
) -> anyhow::Result<EditorRunResult> {
    let source = source.map(|v| v.into());
    let mut siv = cursive::default();
    let initial = text_to_edit.clone();
    let need_save_answer = Arc::new(AtomicBool::new(false));
    let content = Arc::new(Mutex::new(text_to_edit.clone()));
    let on_save = Arc::new(on_save);

    siv.load_toml(include_str!("./theme.toml"))
        .expect("Failed to load theme");

    siv.clear_global_callbacks(Event::CtrlChar('c'));

    let on_save = on_save.clone();
    siv.add_global_callback(Event::CtrlChar('s'), move |s| {
        let content = s
            .find_name::<EditorLayer>("editor_layer")
            .expect(VIEW_NOT_FOUND)
            .get_content();

        if let Err(e) = on_save(content) {
            s.add_layer(ErrorLayer::new(e.to_string(), |s| {
                s.pop_layer();
            }));
            return;
        }

        let mut editor_layer = s
            .find_name::<EditorLayer>("editor_layer")
            .expect(VIEW_NOT_FOUND);
        editor_layer.set_modified(false);
    });

    let nsa = need_save_answer.clone();
    siv.add_global_callback(Event::CtrlChar('x'), move |s| {
        let changed = s
            .find_name::<EditorLayer>("editor_layer")
            .expect(VIEW_NOT_FOUND)
            .get_modified();
        if !changed {
            nsa.store(false, Ordering::Relaxed);
            s.quit();
            return;
        }

        let nsa1 = nsa.clone();
        let nsa2 = nsa.clone();
        s.add_layer(ConfirmSaveLayer::new(
            move |s| {
                nsa1.store(true, Ordering::Relaxed);
                s.pop_layer();
                s.quit();
            },
            move |s| {
                nsa2.store(false, Ordering::Relaxed);
                s.pop_layer();
                s.quit();
            },
        ));
    });

    siv.add_global_callback(Event::Key(F2), |s| {
        let mut editor_layer = s
            .find_name::<EditorLayer>("editor_layer")
            .expect(VIEW_NOT_FOUND);
        editor_layer.toggle_selection_mode();
        _ = std::io::stdout().set_selection(editor_layer.get_selection_mode());
    });

    siv.add_global_callback(Event::CtrlChar('r'), |s| {
        s.find_name::<EditorLayer>("editor_layer")
            .expect(VIEW_NOT_FOUND)
            .toggle_preview_mode();
    });

    let c = content.clone();
    siv.add_fullscreen_layer(
        EditorLayer::new(initial.as_str(), source, false, move |(s, changed)| {
            let mut editor_layer = s
                .find_name::<EditorLayer>("editor_layer")
                .expect(VIEW_NOT_FOUND);
            let content = editor_layer.get_content();
            let mut c = c.lock().expect("Failed to lock");
            *c = content;

            editor_layer.set_modified(!changed);
        })
        .with_name("editor_layer"),
    );

    let mut runner = siv.into_runner();

    std::io::stdout().set_selection(false)?;

    runner.run();

    Ok(EditorRunResult::new(
        content.lock().expect("Failed to lock").clone(),
        need_save_answer.load(Ordering::Relaxed),
    ))
}
