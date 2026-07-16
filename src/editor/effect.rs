use crate::editor::outcome::EditorOutcome;

#[derive(Debug, Clone)]
pub(crate) enum Effect {
    Finish(EditorOutcome),
}
