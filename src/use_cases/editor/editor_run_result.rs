pub(crate) struct EditorRunResult {
    content: String,
    need_save: bool,
}

impl EditorRunResult {
    pub fn new(content: String, need_save: bool) -> Self {
        Self { content, need_save }
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn need_save(&self) -> bool {
        self.need_save
    }
}
