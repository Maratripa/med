use crate::{cursor::Cursor, terminal::Size};

pub struct View {
    size: Size,
    pub dx: usize,
    pub dy: usize,
}

impl View {
    pub fn new() -> Self {
        Self {
            size: Size {
                width: 0,
                height: 0,
            },
            dx: 0,
            dy: 0,
        }
    }

    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn adjust_offset(&mut self, cursor: &Cursor) {
        if cursor.y > self.dy + self.size.height as usize {
            self.dy = cursor.y - self.size.height as usize;
        } else if cursor.y < self.dy {
            self.dy = cursor.y;
        }
    }
}
