use bevy_ecs::system::{Res, ResMut};

use super::StatusBuffer;
use crate::output::buffer::OutputBuffer;

pub fn draw_status_bar(status: Res<StatusBuffer>, mut buffer: ResMut<OutputBuffer>) {
    buffer.push_str(&status.value);
}
