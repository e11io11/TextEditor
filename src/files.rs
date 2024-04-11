use crate::text_zone::TextContent;

pub(crate) struct File {
    pub path: Option<String>,
    pub content: TextContent,
}

pub(crate) struct FileContext {
    files: Vec<File>,
    current: usize,
}

impl File {
    pub fn new() -> Self {
        File {
            path: None,
            content: TextContent::new(),
        }
    }
}

impl FileContext {
    pub fn new() -> Self {
        FileContext {
            files: vec![File::new()],
            current: 0,
        }
    }

    pub fn current(&mut self) -> &mut File {
        &mut self.files[self.current]
    }

    pub fn _set_current_content(&mut self, content: TextContent) {
        self.files[self.current].content = content;
    }

    pub fn set_current_path(&mut self, path: String) {
        self.files[self.current].path = Some(path);
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn select_last(&mut self) {
        self.current = self.files.len() - 1;
    }
}
