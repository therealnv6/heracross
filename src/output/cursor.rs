use super::OutputSize;
use crossterm::{cursor, queue};
use std::io::Result;

pub struct CursorBuffer {
    cursor_x: usize,
    cursor_y: usize,
    offset: CursorOffset,
    output_size: OutputSize,
}

pub struct CursorOffset {
    pub(crate) row: usize,
    pub(crate) column: usize,
}

impl CursorBuffer {
    pub fn new(output_size: OutputSize) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            offset: CursorOffset { row: 0, column: 0 },
            output_size,
        }
    }

    pub fn update_pos(&mut self, direction: char, row_count: usize) {
        // these calls are "saturated" to avoid under and overflows
        match direction {
            'h' => self.cursor_x = self.cursor_x.saturating_sub(1),
            'j' => {
                self.cursor_y = self
                    .cursor_y
                    .saturating_add(1)
                    .min(self.output_size.columns)
                    .min(row_count);
            }
            'k' => self.cursor_y = self.cursor_y.saturating_sub(1),
            'l' => self.cursor_x = self.cursor_x.saturating_add(1).min(self.output_size.rows),
            _ => (),
        }

        self.scroll();
    }

    pub fn scroll(&mut self) {
        let CursorOffset {
            row: mut row_offset,
            column: mut column_offset,
        } = &mut self.offset;

        row_offset = row_offset.min(self.cursor_y);
        column_offset = column_offset.min(self.cursor_x);

        if self.cursor_y >= row_offset + self.output_size.rows {
            row_offset = self.cursor_y - self.output_size.rows + 1;
        }

        if self.cursor_x >= column_offset + self.output_size.columns {
            column_offset = self.cursor_x - self.output_size.columns + 1;
        }
    }

    pub fn queue_hide(&self, writer: &mut impl std::io::Write) -> Result<()> {
        queue!(writer, cursor::Hide, cursor::MoveTo(0, 0))
    }

    pub fn queue_update(&self, writer: &mut impl std::io::Write) -> Result<()> {
        queue!(
            writer,
            cursor::MoveTo(
                self.cursor_x.try_into().unwrap(),
                self.cursor_y.try_into().unwrap()
            ),
            cursor::Show
        )
    }

    pub fn get_offset(&self) -> &CursorOffset {
        return &self.offset;
    }

    pub fn row_offset(&self) -> usize {
        return self.offset.row;
    }
}
