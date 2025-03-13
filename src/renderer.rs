use crate::{document::Document, terminal::Terminal, view::View};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, view: &View, doc: &Document, term: &mut Terminal) {
        term.move_cursor(0, 0).unwrap();

        let view_size = view.size();
        let (view_height, view_width) = (view_size.height as usize, view_size.width as usize);

        let start_idx = doc.text().line_to_char(view.start());

        let len_chars = doc.text().len_chars();

        let end_idx = if let Ok(end) = doc.text().try_line_to_char(view.start() + view_height) {
            let total = end + view_width;
            if total < len_chars { total } else { len_chars }
        } else {
            len_chars - 1
        };

        let text_slice = doc.text().slice(start_idx..end_idx);
        let len_lines = text_slice.len_lines();

        for i in 0..view_height {
            if i != 0 {
                term.write("\n\r").unwrap();
            }

            if i >= len_lines {
                break;
            }

            let mut line_slice = text_slice.line(i).to_string();

            let line_len = line_slice.trim_end_matches(&['\r', '\n']).len();
            line_slice.truncate(line_len);

            term.write(&line_slice).unwrap();
        }

        term.flush().unwrap();
    }
}
