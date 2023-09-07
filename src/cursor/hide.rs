use super::CursorHideEvent;
use crate::output::buffer::OutputBuffer;
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::ResMut,
};
use crossterm::{cursor, queue};

pub fn handle_cursor_hide(
    mut events: EventReader<CursorHideEvent>,
    mut buffer: ResMut<OutputBuffer>,
) {
    for _ in events.iter() {
        queue!(buffer, cursor::Hide, cursor::MoveTo(0, 0)).expect("Couldn't update cursor");
    }
}

// dispatch event wrappers
pub fn dispatch_hide(mut writer: EventWriter<CursorHideEvent>) {
    writer.send_default();
}
