use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct EditorDocument {
    path: Option<PathBuf>,
}

impl EditorDocument {
    pub(crate) fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }

    pub(crate) fn path_label(&self) -> String {
        self.path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "Untitled".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::EditorDocument;

    #[test]
    fn untitled_documents_have_a_label() {
        let document = EditorDocument::new(None);
        assert_eq!(document.path_label(), "Untitled");
    }
}
