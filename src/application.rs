use anyhow::Result;
use crossterm::event::{Event, EventStream};

use tokio::select;
use tokio_stream::StreamExt;

use crate::{
    document::Document,
    editor::Editor,
    terminal::{Size, Terminal},
    view::View,
};

pub struct Application {
    terminal: Terminal,
    editor: Editor,
    view: View,
}

impl Application {
    pub fn new(args: Vec<String>, terminal: Terminal) -> Self {
        let mut editor = Editor::new();
        let mut view = View::new();

        if args.len() > 1 {
            if let Ok(doc) = Document::new_from_path(args[1].to_owned().into()) {
                editor.open_document(doc);
            }
        }

        let mut terminal_size = *terminal.size();
        terminal_size.height -= 1; // Remove a line for statusline

        view.resize(terminal_size);

        Self {
            terminal,
            editor,
            view,
        }
    }

    async fn event_loop(&mut self) -> Result<()> {
        let mut reader = EventStream::new();

        loop {
            if self.editor.should_quit() {
                break;
            }

            select! {
                biased;
                // some
                Some(event) = reader.next() => {
                    self.handle_terminal_event(event).await;
                }
            }
        }

        Ok(())
    }

    async fn handle_terminal_event(&mut self, event: std::io::Result<Event>) {
        let event = event.expect("Couldn't unwarp Event");

        let should_render = match event {
            Event::Resize(w, h) => {
                self.resize(w, h);
                true
            }
            Event::Key(key_event) => self.editor.handle_key(key_event),
            _ => false,
        };

        if should_render && !self.editor.should_quit() {
            self.render();
        }
    }

    fn render(&mut self) {
        self.view.adjust_offset(&self.editor.cursor());
        let view_size = self.view.size();
        let (view_height, view_width) = (view_size.height as usize, view_size.width as usize);

        let lines_iter = self
            .editor
            .document()
            .text()
            .lines()
            .skip(self.view.dy)
            .take(view_height);

        self.terminal.clear_screen().unwrap();
        for (row, line) in lines_iter.enumerate() {
            self.terminal.move_cursor(0, row as u16).unwrap();

            let mut line_str = line.clone().to_string();
            line_str.truncate(view_width);

            self.terminal.write(&line_str).unwrap();
        }

        let (x, y) = self.editor.cursor().get_position_as_tuple();
        self.terminal
            .move_cursor((x - self.view.dx) as u16, (y - self.view.dy) as u16)
            .unwrap();
        self.terminal.flush().unwrap();
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.view.resize(Size {
            width: w,
            height: h - 1, // Remove a line for statusline
        });
    }

    pub async fn run(&mut self) -> Result<()> {
        self.terminal.setup()?;

        // self.event_loop().await?;
        //
        self.render();
        self.event_loop().await?;

        self.close()?;
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        self.terminal.restore()?;
        Ok(())
    }
}
