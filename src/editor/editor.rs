use crate::editor::{
    application::EditorApplication, editor_config::EditorConfig, outcome::EditorOutcome, runtime,
    state::EditorState,
};

pub(crate) async fn run_editor(config: EditorConfig) -> anyhow::Result<EditorOutcome> {
    let Some(path) = config.path() else {
        let state = EditorState::new(None, String::new());
        return runtime::run(EditorApplication::new(state)).await;
    };

    let text = tokio::fs::read_to_string(path).await.unwrap_or_default();
    let state = EditorState::new(Some(path.to_path_buf()), text);
    runtime::run(EditorApplication::new(state)).await
}
