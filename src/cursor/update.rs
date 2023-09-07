use bevy_ecs::prelude::*;
use crossterm::{cursor, execute};

use crate::{
    input::data::RowBuffer,
    output::{buffer::OutputBuffer, OutputSize},
};

use super::{CursorDirection, CursorMoveEvent, CursorPosition};

pub fn handle_cursor_move(
    row_buffer: Res<RowBuffer>,
    output_size: Res<OutputSize>,
    mut position: ResMut<CursorPosition>,
    mut events: EventReader<CursorMoveEvent>,
) {
    for CursorMoveEvent { direction } in events.iter() {
        match direction {
            CursorDirection::Left => position.x = position.x.saturating_sub(1),
            CursorDirection::Down => {
                position.y = position
                    .y
                    .saturating_add(1)
                    .min(output_size.columns)
                    .min(row_buffer.rows_count());
            }
            CursorDirection::Up => position.y = position.y.saturating_sub(1),
            CursorDirection::Right => {
                position.x = position.x.saturating_add(1).min(output_size.rows)
            }
        }
    }
}

pub fn update_cursor(mut buffer: ResMut<OutputBuffer>, position: Res<CursorPosition>) {
    if position.is_changed() {
        execute!(
            buffer,
            cursor::MoveTo(
                position.x.try_into().unwrap(),
                position.y.try_into().unwrap()
            ),
            cursor::Show
        )
        .expect("Couldn't update cursor");
    }
}
