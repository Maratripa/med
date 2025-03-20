use crossterm::{
    QueueableCommand,
    cursor::{self, MoveTo},
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::Write;

use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

#[derive(Debug)]
struct Patch {
    cell: Cell,
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Buffer {
    cells: Vec<Cell>,
    w: usize,
    h: usize,
}

impl Buffer {
    fn new(w: usize, h: usize) -> Self {
        Self {
            cells: vec![Cell::default(); w * h],
            w,
            h,
        }
    }

    fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    fn content(&self) -> &[Cell] {
        &self.cells
    }

    fn resize(&mut self, new_width: usize, new_height: usize) {
        self.w = new_width;
        self.h = new_height;
        self.cells.resize(self.w * self.h, Cell::default());
        self.clear();
    }

    fn diff(&self, other: &Self) -> Vec<Patch> {
        self.cells
            .iter()
            .zip(other.cells.iter())
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(i, (_, cell))| {
                let x = i % self.w;
                let y = i / self.w;
                Patch {
                    cell: cell.clone(),
                    x,
                    y,
                }
            })
            .collect()
    }

    fn flush(&self, out: &mut Box<dyn Write>) -> std::io::Result<()> {
        let mut fg_curr = Color::Reset;
        let mut bg_curr = Color::Reset;
        out.queue(Clear(ClearType::All))?;
        out.queue(SetForegroundColor(fg_curr))?;
        out.queue(SetBackgroundColor(bg_curr))?;
        out.queue(MoveTo(0, 0))?;
        for Cell { ch, fg, bg } in self.cells.iter() {
            if &fg_curr != fg {
                fg_curr = *fg;
                out.queue(SetForegroundColor(fg_curr))?;
            }
            if &bg_curr != bg {
                bg_curr = *bg;
                out.queue(SetBackgroundColor(bg_curr))?;
            }
            out.queue(Print(ch))?;
        }
        out.flush()?;
        Ok(())
    }
}

pub struct Terminal {
    out: Box<dyn Write>,
    pub buf_curr: Buffer,
    pub buf_prev: Buffer,
}

impl Terminal {
    pub fn new(out: Box<dyn Write>) -> Result<Self> {
        let (w, h) = terminal::size()?;
        Ok(Self {
            out,
            buf_curr: Buffer::new(w as usize, h as usize),
            buf_prev: Buffer::new(w as usize, h as usize),
        })
    }

    pub fn width(&self) -> usize {
        self.buf_curr.w
    }

    pub fn height(&self) -> usize {
        self.buf_curr.h
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.buf_curr.resize(new_width, new_height);
        self.buf_prev.resize(new_width, new_height);
        self.buf_prev.flush(&mut self.out).unwrap();
    }

    pub fn put_cell(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        let width = self.width();
        if let Some(cell) = self.buf_curr.cells.get_mut(y * width + x) {
            *cell = Cell { ch, fg, bg }
        }
    }

    pub fn put_cells(&mut self, x: usize, y: usize, chs: String, fg: Color, bg: Color) {
        let start = y * self.width() + x;
        for (offset, ch) in chs.chars().enumerate() {
            if let Some(cell) = self.buf_curr.cells.get_mut(start + offset) {
                *cell = Cell { ch, fg, bg };
            } else {
                break;
            }
        }
    }

    pub fn queue_draw(&mut self) -> std::io::Result<()> {
        let patches = self.buf_prev.diff(&self.buf_curr);

        let mut curr_bg = Color::Reset;
        let mut curr_fg = Color::Reset;

        let mut x_prev = 0;
        let mut y_prev = 0;

        for Patch {
            cell: Cell { ch, bg, fg },
            x,
            y,
        } in patches.iter()
        {
            if &(x_prev + 1) != x || &y_prev != y {
                self.out.queue(MoveTo(*x as u16, *y as u16))?;
            }
            x_prev = *x;
            y_prev = *y;
            if bg != &curr_bg {
                curr_bg = *bg;
                self.out.queue(SetBackgroundColor(curr_bg))?;
            }
            if fg != &curr_fg {
                curr_fg = *fg;
                self.out.queue(SetForegroundColor(curr_fg))?;
            }

            self.out.queue(Print(ch))?;
        }

        std::mem::swap(&mut self.buf_curr, &mut self.buf_prev);
        self.buf_curr.clear();

        Ok(())
    }

    pub fn setup(&mut self) -> Result<()> {
        execute!(
            self.out,
            terminal::EnterAlternateScreen,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        )?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(
            self.out,
            PopKeyboardEnhancementFlags,
            terminal::LeaveAlternateScreen
        )?;
        Ok(())
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        queue!(self.out, cursor::MoveTo(x, y))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.out.flush()?;
        Ok(())
    }
}
