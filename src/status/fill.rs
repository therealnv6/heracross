use bevy_ecs::prelude::*;
use crossterm::style::Attribute;

use crate::input::InputMode;

use super::StatusBuffer;

pub fn fill_bar(mut status_bar: ResMut<StatusBuffer>, input_mode: Res<InputMode>) {
    let mut bar_value = String::new();

    bar_value.push_str(&input_mode.to_string());

    // reverse the contents of the bar value
    status_bar.value = wrap_reversed(bar_value);
}

pub fn wrap_reversed(value: String) -> String {
    return Attribute::Reverse.to_string() + &value + &Attribute::Reset.to_string();
}
