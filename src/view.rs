use crate::cursor::Cursor;

pub struct View {
    width: usize,
    height: usize,
    pub dx: usize,
    pub dy: usize,
}

impl View {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            dx: 0,
            dy: 0,
        }
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn adjust_offset(&mut self, cursor: &Cursor) {
        if cursor.y > self.dy + self.height as usize - 1 {
            self.dy = cursor.y - self.height as usize + 1;
        } else if cursor.y < self.dy {
            self.dy = cursor.y;
        }
    }
}
