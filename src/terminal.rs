use crossterm::{
    cursor,
    event::{
        self, Event, KeyEvent, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    execute, queue,
    style::{self, Color, Print},
    terminal,
};
use std::io::{Stdout, Write, stdout};

use anyhow::Result;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    stdout: Stdout,
    size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let size = terminal::size()?;
        Ok(Self {
            stdout: stdout(),
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn setup(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        )?;
        terminal::enable_raw_mode()?;
        self.clear_screen()?;
        self.move_cursor(0, 0)?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(
            self.stdout,
            PopKeyboardEnhancementFlags,
            terminal::LeaveAlternateScreen
        )?;
        Ok(())
    }

    pub fn read_key() -> Result<KeyEvent> {
        loop {
            if let Event::Key(key) = event::read()? {
                return Ok(key);
            }
        }
    }

    pub fn clear_screen(&mut self) -> Result<()> {
        queue!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn clear_current_line(&mut self) -> Result<()> {
        queue!(
            self.stdout,
            terminal::Clear(terminal::ClearType::CurrentLine)
        )?;
        Ok(())
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        queue!(self.stdout, cursor::MoveTo(x, y))?;
        Ok(())
    }

    pub fn write(&mut self, string: &str) -> Result<()> {
        queue!(self.stdout, Print(string))?;
        Ok(())
    }

    pub fn set_bg_color(&mut self, color: Color) -> Result<()> {
        queue!(self.stdout, style::SetBackgroundColor(color))?;
        Ok(())
    }

    pub fn set_fg_color(&mut self, color: Color) -> Result<()> {
        queue!(self.stdout, style::SetForegroundColor(color))?;
        Ok(())
    }

    pub fn reset_color(&mut self) -> Result<()> {
        queue!(self.stdout, style::ResetColor)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;
        Ok(())
    }
}
