use bevy_ecs::system::{Res, ResMut, Resource};
use crossterm::terminal::ClearType;
use crossterm::{execute, queue, terminal};

use std::io::{stdout, Write};

use crate::cursor::CursorOffset;
use crate::input::data::RowBuffer;

use self::buffer::OutputBuffer;

pub mod buffer;

#[derive(Copy, Clone, Resource)]
pub struct OutputSize {
    pub(crate) columns: usize,
    pub(crate) rows: usize,
}

impl Default for OutputSize {
    fn default() -> Self {
        terminal::size()
            .map(|(x, y)| OutputSize {
                columns: x as usize,
                rows: y as usize,
            })
            .unwrap()
    }
}

pub fn draw_rows(
    size: Res<OutputSize>,
    rows: ResMut<RowBuffer>,
    cursor_offset: Res<CursorOffset>,
    mut buffer: ResMut<OutputBuffer>,
) {
    let OutputSize {
        columns,
        rows: row_size,
    } = *size;

    for current in 0..row_size {
        let row = current + cursor_offset.row;

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
            } = *cursor_offset;

            let len = row.len().min(columns);
            let start = column_offset.max(0);

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

pub fn clear() {
    execute!(stdout(), terminal::Clear(ClearType::All)).expect("Could not clear stdout!");
}
