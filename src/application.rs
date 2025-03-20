use anyhow::Result;
use crossterm::event::{Event, EventStream};
use crossterm::style::Color;
use std::io::stdout;

use tokio::select;
use tokio_stream::StreamExt;

use crate::{document::Document, editor::Editor, terminal::Terminal, view::View};

pub struct Application {
    terminal: Terminal,
    editor: Editor,
    view: View,
    statusline: String,
}

impl Application {
    pub fn new(args: Vec<String>) -> Self {
        let mut editor = Editor::new();

        if args.len() > 1 {
            if let Ok(doc) = Document::new_from_path(args[1].to_owned().into()) {
                editor.open_document(doc);
            }
        }

        let terminal =
            Terminal::new(Box::new(stdout())).expect("Failed to initialize Terminal struct");

        let mut view = View::new();
        view.resize(terminal.width(), terminal.height());

        Self {
            terminal,
            editor,
            view,
            statusline: String::from("Welcome to MED!"),
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
            self.render().await;
        }
    }

    async fn render(&mut self) {
        self.view.adjust_offset(&self.editor.cursor());

        let lines_iter = self
            .editor
            .document()
            .text()
            .lines()
            .skip(self.view.dy)
            .take(self.view.height());

        for (row, line) in lines_iter.enumerate() {
            let mut line_str = line.clone().to_string();
            let len = line_str.trim_end_matches(&['\r', '\n']).len();
            line_str.truncate(std::cmp::min(self.view.width(), len));

            self.terminal
                .put_cells(0, row, line_str, Color::Reset, Color::Reset);
        }
        self.terminal.queue_draw().unwrap();

        let (x, y) = self.editor.cursor().get_position_as_tuple();
        self.terminal
            .move_cursor((x - self.view.dx) as u16, (y - self.view.dy) as u16)
            .unwrap();
        self.terminal.flush().unwrap();
    }

    fn resize(&mut self, w: u16, h: u16) {
        let w = w as usize;
        let h = h as usize;

        self.terminal.resize(w, h);
        self.view.resize(w, h);
    }

    pub async fn run(&mut self) -> Result<()> {
        self.terminal.setup()?;

        self.render().await;
        self.event_loop().await?;

        self.close()?;
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        self.terminal.restore()?;
        Ok(())
    }
}
