use crate::input::data::{Row, RowBuffer};

use super::OutputSize;
use bevy_ecs::{
    event::{Event, EventReader},
    system::{Res, ResMut, Resource},
};
use crossterm::{cursor, queue};
use std::io::Result;

#[derive(Clone, Resource)]
pub struct CursorBuffer {
    cursor_x: usize,
    cursor_y: usize,
    offset: CursorOffset,
    output_size: OutputSize,
    render_dist: usize,
}

#[derive(Clone)]
pub struct CursorOffset {
    pub(crate) row: usize,
    pub(crate) column: usize,
}

#[derive(Event)]
pub struct CursorMoveEvent {
    direction: CursorDirection,
}

pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CursorBuffer {
    pub fn new(output_size: OutputSize) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            render_dist: 0,
            offset: CursorOffset { row: 0, column: 0 },
            output_size,
        }
    }

    pub fn update_pos(&mut self, direction: char, row_count: usize, rows: &RowBuffer) {
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

        self.scroll(rows);
    }

    pub fn scroll(&mut self, rows: &RowBuffer) {
        let CursorOffset {
            row: mut row_offset,
            column: mut column_offset,
        } = &mut self.offset;

        row_offset = row_offset.min(self.cursor_y);
        column_offset = column_offset.min(self.render_dist);

        if self.cursor_y < rows.rows_count() {
            self.render_dist = rows
                .get_row_at(self.cursor_y)
                .get_len_with_offset(self.cursor_y);
        }

        if self.cursor_y >= row_offset + self.output_size.rows {
            self.offset.row = self.cursor_y - self.output_size.rows + 1;
        }

        // we use render_dist instead of cursor_x, to properly render the correct characters.
        if self.render_dist >= column_offset + self.output_size.columns {
            self.offset.column = self.render_dist - self.output_size.columns + 1;
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

pub fn handle_cursor_move(
    row_buffer: Res<RowBuffer>,
    mut buffer: ResMut<CursorBuffer>,
    mut events: EventReader<CursorMoveEvent>,
) {
    for CursorMoveEvent { direction } in events.iter() {
        match direction {
            CursorDirection::Left => buffer.cursor_x = buffer.cursor_x.saturating_sub(1),
            CursorDirection::Down => {
                buffer.cursor_y = buffer
                    .cursor_y
                    .saturating_add(1)
                    .min(buffer.output_size.columns)
                    .min(row_buffer.rows_count());
            }
            CursorDirection::Up => buffer.cursor_y = buffer.cursor_y.saturating_sub(1),
            CursorDirection::Right => {
                buffer.cursor_x = buffer
                    .cursor_x
                    .saturating_add(1)
                    .min(buffer.output_size.rows)
            }
        }
    }
}
