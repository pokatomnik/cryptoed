mod editor_buffer;

use std::path::PathBuf;

use crate::editor::document::EditorDocument;

pub(crate) use editor_buffer::EditorBuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Focus {
    Editor,
}

#[derive(Debug, Clone)]
pub(crate) struct EditorState {
    document: EditorDocument,
    buffer: EditorBuffer,
    focus: Focus,
}

impl EditorState {
    pub(crate) fn new(path: Option<PathBuf>, text: String) -> Self {
        Self {
            document: EditorDocument::new(path),
            buffer: EditorBuffer::new(text),
            focus: Focus::Editor,
        }
    }

    pub(crate) fn document(&self) -> &EditorDocument {
        &self.document
    }

    pub(crate) fn buffer(&self) -> &EditorBuffer {
        &self.buffer
    }

    pub(crate) fn buffer_mut(&mut self) -> &mut EditorBuffer {
        &mut self.buffer
    }

    pub(crate) fn focus(&self) -> Focus {
        self.focus
    }
}
