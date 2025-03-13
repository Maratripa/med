use anyhow::Result;
use crossterm::event::{Event, EventStream};

use std::io::Write;

use tokio::select;
use tokio_stream::StreamExt;

use crate::{
    document::Document,
    editor::Editor,
    renderer::Renderer,
    terminal::{Size, Terminal},
    view::View,
};

pub struct Application {
    terminal: Terminal,
    editor: Editor,
    view: View,
    renderer: Renderer,
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
            renderer: Renderer::new(),
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
            Event::Key(key_event) => self.editor.handle_key(&mut self.terminal, key_event),
            _ => false,
        };

        if should_render && !self.editor.should_quit() {
            self.render();
        }
    }

    fn render(&mut self) {
        self.renderer
            .render(&self.view, self.editor.document(), &mut self.terminal);

        let (x, y) = self.editor.cursor().get_position_as_tuple();
        self.terminal.move_cursor(x as u16, y as u16).unwrap();
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
