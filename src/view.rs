use crate::terminal::Size;

pub struct View {
    first_line: usize,
    size: Size,
}

impl View {
    pub fn new() -> Self {
        Self {
            first_line: 0,
            size: Size {
                width: 0,
                height: 0,
            },
        }
    }

    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    pub fn start(&self) -> usize {
        self.first_line
    }

    pub fn size(&self) -> Size {
        self.size
    }
}
