use bevy_ecs::system::{Res, ResMut, Resource};
use crossterm::style::Attribute;

use crate::input::InputMode;

use super::Output;

#[derive(Clone, Resource)]
pub struct StatusBar {
    value: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn draw(&mut self, output: Output) -> String {
        let mut builder = String::new();
        let input_mode = InputMode::Insert;
        let attribute_len =
            Attribute::Reverse.to_string().len() + Attribute::Reset.to_string().len();

        builder.push_str(&Attribute::Reverse.to_string());
        builder.push_str(&input_mode.to_string());
        (0..output.win_size.columns - attribute_len).for_each(|_| builder.push(' '));
        builder.push_str(&Attribute::Reset.to_string());

        self.value = builder.clone();
        builder
    }
}

pub fn fill_bar(mut status_bar: ResMut<StatusBar>, input_mode: Res<InputMode>) {
    let mut bar_value = String::new();

    bar_value.push_str(&input_mode.to_string());

    // reverse the contents of the bar value
    status_bar.value = wrap_reversed(bar_value);
}

pub fn wrap_reversed(value: String) -> String {
    return Attribute::Reverse.to_string() + &value + &Attribute::Reset.to_string();
}
