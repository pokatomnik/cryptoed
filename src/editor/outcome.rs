#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinishReason {
    SaveAndExit,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EditorOutcome {
    reason: FinishReason,
    text: String,
}

impl EditorOutcome {
    pub(crate) fn new(reason: FinishReason, text: String) -> Self {
        Self { reason, text }
    }

    pub(crate) fn reason(&self) -> FinishReason {
        self.reason
    }

    pub(crate) fn text(&self) -> &str {
        &self.text
    }
}
