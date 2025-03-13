pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn get_position_as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
