use std::{fs, path::Path};

use bevy_ecs::system::Resource;

pub const TAB_SIZE: usize = 4;

#[derive(Clone)]
pub struct Row {
    contents: Box<str>,
    render: String,
}

impl Row {
    fn new(contents: Box<str>, render: String) -> Self {
        Self { contents, render }
    }

    pub fn get_len_with_offset(&self, offset: usize) -> usize {
        if offset > self.contents.len() {
            return self.contents.len();
        }

        self.contents[..offset].chars().fold(0, |acc, current| {
            acc + if current == '\t' {
                (TAB_SIZE - 1) - (acc % TAB_SIZE) + 1
            } else {
                1
            }
        })
    }
}

#[derive(Clone, Resource)]
pub struct RowBuffer {
    rows: Vec<Row>,
    name: String,
}

impl RowBuffer {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            name: String::from("Empty Buffer"),
        }
    }

    pub fn render_row_at(&mut self, y: usize) {
        let row = self.rows.get_mut(y);

        if let Some(row) = row {
            Self::render_row(row);
        }
    }

    pub fn get_buffer_name(&self) -> &str {
        return &self.name;
    }

    pub fn render_row(row: &mut Row) {
        // Calculate the capacity needed for the `render` String
        let capacity = row
            .contents
            .chars()
            .map(|c| if c == '\t' { TAB_SIZE } else { 1 })
            .sum();

        // Create the `render` String with the calculated capacity
        row.render = String::with_capacity(capacity);

        // Iterate over the characters in `contents`
        for current in row.contents.chars() {
            if current == '\t' {
                // Replace tabs with spaces
                row.render.push(' ');

                // Calculate the number of spaces needed to reach the next tab stop
                let next_tab_diff = TAB_SIZE - (row.render.len() % TAB_SIZE);

                // Append the required number of spaces
                (0..next_tab_diff).for_each(|_| row.render.push(' '));
            } else {
                // Copy other characters as-is
                row.render.push(current);
            }
        }
    }

    pub fn get_char_count(&self) -> usize {
        return self.rows.iter().map(|row| row.contents.len()).sum();
    }

    pub fn get_render(&self, y: usize) -> &String {
        &self.rows[y].render
    }

    pub fn get_row_at(&self, y: usize) -> &Row {
        &self.rows[y]
    }

    pub fn rows_count(&self) -> usize {
        self.rows.len()
    }
}

impl TryFrom<&Path> for RowBuffer {
    type Error = std::io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let contents = fs::read_to_string(value)?;

        Ok(Self {
            name: value
                .file_name()
                .expect("Could not get file name from path")
                .to_string_lossy()
                .to_string(),
            rows: contents
                .lines()
                .map(|it| {
                    let mut row = Row::new(it.into(), it.to_string());

                    Self::render_row(&mut row);
                    return row;
                })
                .collect(),
        })
    }
}
