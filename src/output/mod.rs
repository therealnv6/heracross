use bevy_ecs::system::{Res, ResMut, Resource};
use crossterm::terminal::{ClearType, WindowSize};
use crossterm::{execute, queue, terminal};

use std::io::{stdout, Result, Write};
use std::path::Path;

use crate::input::data::RowBuffer;

use self::buffer::OutputBuffer;
use self::cursor::{CursorBuffer, CursorOffset};
use self::status::StatusBar;

pub mod buffer;
pub mod cursor;
pub mod status;

#[derive(Clone)]
pub struct Output {
    win_size: OutputSize,
    rows: RowBuffer,
    cursor: CursorBuffer,
    buffer: OutputBuffer,
    status_bar: StatusBar,
}

#[derive(Copy, Clone, Resource)]
pub struct OutputSize {
    columns: usize,
    rows: usize,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| OutputSize {
                columns: x as usize,
                rows: y as usize,
            })
            .unwrap();

        Self {
            win_size: win_size.clone(),
            status_bar: StatusBar::new(),
            cursor: CursorBuffer::new(win_size.clone()),
            rows: RowBuffer::try_from(Path::new(
                "/home/riven/projects/rust/heracross/src/output/mod.rs",
            ))
            .expect("meow"),
            buffer: OutputBuffer::new(),
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        self.update_cursor()
    }

    pub fn move_cursor(&mut self, direction: char) {
        self.cursor
            .update_pos(direction, self.rows.rows_count(), &self.rows);
    }

    pub fn update_cursor(&mut self) -> Result<()> {
        self.cursor.queue_update(&mut self.buffer)
    }

    pub fn draw_rows(&mut self) -> Result<()> {
        let OutputSize { columns, rows } = self.win_size;

        for current in 0..rows {
            let row = current + self.cursor.row_offset();

            if self.rows.rows_count() == 0 || row >= self.rows.rows_count() {
                if current == rows / 3 {
                    let mut welcome = String::from("Heracross");
                    if welcome.len() > rows {
                        welcome.truncate(rows)
                    }

                    let mut padding = (columns - welcome.len()) / 2;

                    if padding != 0 {
                        self.push('~');
                        padding -= 1;
                    }

                    (0..padding).for_each(|_| self.push(' '));
                    self.push_str(&welcome);
                } else {
                    self.push('~');
                }
            } else {
                let row = self.rows.get_render(row);

                let CursorOffset {
                    column: column_offset,
                    ..
                } = self.cursor.get_offset();

                let len = row.len().min(columns);
                let start = *column_offset.max(&0);

                self.buffer.push_str(&row[start..start + len])
            }

            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();

            self.push_str("\r\n");
            stdout().flush()?;
        }

        Ok(())
    }

    pub fn push(&mut self, char: char) {
        self.buffer.push(char);
    }

    pub fn push_str(&mut self, data: &str) {
        self.buffer.push_str(data);
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.cursor.queue_hide(&mut self.buffer)?;

        self.clear()?;
        self.draw_rows()?;

        {
            let contents = self.status_bar.draw(self.clone());
            self.push_str(&contents);
        }

        self.update_cursor()?;

        // we want to flush the buffer as last!
        self.buffer.flush()
    }
}

pub fn draw_rows(
    size: Res<OutputSize>,
    rows: ResMut<RowBuffer>,
    cursor: ResMut<CursorBuffer>,
    mut buffer: ResMut<OutputBuffer>,
) {
    let OutputSize {
        columns,
        rows: row_size,
    } = *size;

    for current in 0..row_size {
        let row = current + cursor.row_offset();

        if rows.rows_count() == 0 || row >= rows.rows_count() {
            if current == row_size / 3 {
                let mut welcome = String::from("Heracross");
                if welcome.len() > row_size {
                    welcome.truncate(row_size)
                }

                let mut padding = (columns - welcome.len()) / 2;

                if padding != 0 {
                    buffer.push('~');
                    padding -= 1;
                }

                (0..padding).for_each(|_| buffer.push(' '));
                buffer.push_str(&welcome);
            } else {
                buffer.push('~');
            }
        } else {
            let row = rows.get_render(row);

            let CursorOffset {
                column: column_offset,
                ..
            } = cursor.get_offset();

            let len = row.len().min(columns);
            let start = *column_offset.max(&0);

            buffer.push_str(&row[start..start + len])
        }

        queue!(buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();

        buffer.push_str("\r\n");
        stdout().flush().expect("Could not flush stdout!");
    }
}

pub fn flush_buffer(mut buffer: ResMut<OutputBuffer>) {
    buffer.flush().expect("Could not flush buffer!");
}
