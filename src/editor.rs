use crate::document::Document;

enum Mode {
    Normal,
    Insert,
}

pub struct Editor {
    should_quit: bool,
    mode: Mode,
    document: Document,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            mode: Mode::Normal,
            document: Document::new(),
        }
    }

    pub fn open_document(&mut self, doc: Document) {
        self.document = doc;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn set_should_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn document(&self) -> &Document {
        &self.document
    }
}
