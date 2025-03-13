use std::{
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Result;
use ropey::Rope;

pub struct Document {
    path: Option<PathBuf>,
    text: Rope,
}

impl Document {
    pub fn new() -> Self {
        Self {
            path: None,
            text: Rope::new(),
        }
    }

    pub fn new_from_path(path: PathBuf) -> Result<Self> {
        let file = std::fs::File::open(&path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader)?;

        Ok(Self {
            path: Some(path),
            text: rope,
        })
    }

    pub fn set_path(&mut self, path: &Path) {
        let path = path.to_owned();
        self.path = Some(path);
    }

    pub fn text(&self) -> &Rope {
        &self.text
    }
}
