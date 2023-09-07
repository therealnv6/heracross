use super::{CursorOffset, CursorPosition};
use crate::{input::data::RowBuffer, output::OutputSize};
use bevy_ecs::prelude::*;

pub fn scroll(
    mut offset: ResMut<CursorOffset>,
    cursor_pos: Res<CursorPosition>,
    output_size: Res<OutputSize>,
    rows: Res<RowBuffer>,
) {
    let CursorOffset {
        row: row_offset,
        column: column_offset,
        render: render_offset,
    } = *offset;

    offset.row = row_offset.min(cursor_pos.y);
    offset.column = column_offset.min(render_offset);

    if cursor_pos.y < rows.rows_count() {
        offset.render = rows
            .get_row_at(cursor_pos.y)
            .get_len_with_offset(cursor_pos.y);
    }

    if cursor_pos.y >= row_offset + output_size.rows {
        offset.row = cursor_pos.y - output_size.rows + 1;
    }

    // we use render_dist instead of cursor_x, to properly render the correct characters.
    if render_offset >= column_offset + output_size.columns {
        offset.column = render_offset - output_size.columns + 1;
    }
}
