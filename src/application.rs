use anyhow::Result;
use crossterm::event::{Event, EventStream};

use tokio::select;
use tokio_stream::StreamExt;

use crate::{
    document::Document, editor::Editor, renderer::Renderer, terminal::Terminal, view::View,
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

        view.resize(*terminal.size());

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
            Event::Resize(w, h) => true,
            Event::Key(key_event) => {
                self.editor.set_should_quit();
                false
            }
            _ => false,
        };

        if should_render && !self.editor.should_quit() {
            self.render();
        }
    }

    fn render(&mut self) {
        self.renderer
            .render(&self.view, self.editor.document(), &mut self.terminal);
    }

    pub async fn run(&mut self) -> Result<()> {
        self.terminal.setup()?;

        let _ = Terminal::read_key()?;
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
