use std::io::Write;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{cursor::Cursor, document::Document, terminal::Terminal};

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Normal,
    Insert,
}

pub struct Editor {
    should_quit: bool,
    mode: Mode,
    document: Document,
    cursor: Cursor,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            mode: Mode::Normal,
            document: Document::new(),
            cursor: Cursor::new(),
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

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn handle_key(&mut self, terminal: &mut Terminal, key_event: KeyEvent) -> bool {
        // let mut log = std::fs::OpenOptions::new()
        //     .write(true)
        //     .append(true)
        //     .open("log")
        //     .unwrap();
        // log.write_all(format!("{:?}\n", key_event).as_bytes())
        //     .unwrap();

        match (key_event.code, key_event.modifiers, self.mode) {
            (KeyCode::Esc, KeyModifiers::CONTROL, _) => {
                self.set_should_quit();
                false
            }
            (KeyCode::Esc, _, _) => {
                self.mode = Mode::Normal;
                false
            }
            (KeyCode::Char('i'), _, Mode::Normal) => {
                self.mode = Mode::Insert;
                false
            }
            (KeyCode::Char(c), _, Mode::Insert) => true,
            (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down, _, _) => {
                self.move_cursor(terminal, key_event.code);
                false
            }
            _ => false,
        }
    }

    fn move_cursor(&mut self, terminal: &mut Terminal, key_code: KeyCode) {
        let mut update = false;
        match key_code {
            KeyCode::Right => {
                let line_width = if let Some(line) = self.document().text().get_line(self.cursor.y)
                {
                    line.len_chars().saturating_sub(1) // Sub 1 because of newline
                } else {
                    0
                };

                if self.cursor.x < line_width {
                    self.cursor.x += 1;
                // - 1 because the rope adds a line at the end (?)         --v
                } else if self.cursor.y < self.document().text().len_lines() - 1 {
                    self.cursor.y += 1;
                    self.cursor.x = 0;
                }
            }
            KeyCode::Left => {
                if self.cursor.x == 0 {
                    if self.cursor.y == 0 {
                        return;
                    }
                    self.cursor.y -= 1;
                    self.cursor.x = usize::MAX;
                    update = true;
                } else {
                    self.cursor.x -= 1;
                }
            }
            KeyCode::Up => {
                if self.cursor.y == 0 {
                    return;
                }

                self.cursor.y -= 1;
                update = true;
            }
            KeyCode::Down => {
                // - 1 because the rope adds a line at the end (?)  --v
                if self.cursor.y < self.document().text().len_lines() - 1 {
                    self.cursor.y += 1;
                    update = true;
                }
            }
            _ => unreachable!(),
        }

        if update {
            if let Some(line) = self.document().text().get_line(self.cursor.y) {
                // saturating_sub(1) is because the line counts the newline in the width
                self.cursor.x = std::cmp::min(line.len_chars().saturating_sub(1), self.cursor.x);
            } else {
                self.cursor.x = 0;
            }
        }

        terminal
            .move_cursor(self.cursor.x as u16, self.cursor.y as u16)
            .unwrap();
        terminal.flush().unwrap();
    }
}
