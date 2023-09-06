use std::{fs, path::Path};

pub struct RowBuffer {
    row_contents: Vec<Box<str>>,
}

impl RowBuffer {
    pub fn new() -> Self {
        Self {
            row_contents: Vec::new(),
        }
    }

    pub fn rows_count(&self) -> usize {
        self.row_contents.len()
    }

    pub fn get_row(&self, row_num: usize) -> &str {
        &self.row_contents[row_num]
    }
}

impl TryFrom<&Path> for RowBuffer {
    type Error = std::io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let contents = fs::read_to_string(value)?;

        Ok(Self {
            row_contents: contents.lines().map(|it| it.into()).collect(),
        })
    }
}
