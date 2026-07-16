use std::path::{Path, PathBuf};

#[derive(Default)]
pub(crate) struct EditorConfig {
    path: Option<PathBuf>,
}

impl EditorConfig {
    pub fn new() -> Self {
        Self { path: None }
    }

    pub fn with_path(mut self, path: Option<impl Into<PathBuf>>) -> Self {
        self.path = match path {
            Some(v) => Some(v.into()),
            None => None,
        };

        self
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}
