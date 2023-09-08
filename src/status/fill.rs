use bevy_ecs::prelude::*;
use crossterm::style::{Attribute, Color};
use crossterm::style::{StyledContent, Stylize};
use lazy_static::lazy_static;

use crate::input::data::RowBuffer;
use crate::{input::InputMode, output::OutputSize};

use super::StatusBuffer;

pub const GREEN_COLOR: Color = color_rgb(0x98, 0xbe, 0x65);
pub const RED_COLOR: Color = color_rgb(0xec, 0x5f, 0x67);
pub const PINK_COLOR: Color = color_rgb(0xc6, 0x78, 0xdd);
pub const BAR_FOREGROUND: Color = color_rgb(0xbb, 0xc2, 0xcf);
pub const BAR_BACKGROUND: Color = color_rgb(0x20, 0x23, 0x28);

pub const BAR_PADDING: usize = 2;

lazy_static! {
    static ref EMPTY: StyledContent<String> = String::from(" ").stylize();
}

/// Fills the status bar with content based on the provided parameters. Usually called from Bevy's ECS.
///
/// This is not applicable because it gets called from [bevy_ecs::schedule::Schedule], this means
/// it should not get called manually.
///
/// # Arguments
///
/// * `status_bar` - A mutable reference to the `StatusBuffer` struct that represents the status bar.
/// * `row_buffer` - A reference to the `RowBuffer` struct containing row-related information.
/// * `size` - A reference to the `OutputSize` struct representing the terminal output size.
/// * `input_mode` - A reference to the `InputMode` enum representing the current input mode.

pub fn fill_bar(
    mut status_bar: ResMut<StatusBuffer>,
    row_buffer: Res<RowBuffer>,
    size: Res<OutputSize>,
    input_mode: Res<InputMode>,
) {
    let mut bar_value = String::new();

    let (right_size, bar_right_value) = build_bar_part(vec![
        String::from("[UTF-8] ").with(GREEN_COLOR),
        String::from("UNIX ").with(GREEN_COLOR),
    ]);

    let (left_size, bar_left_value) = build_bar_part(vec![
        match input_mode.clone() {
            InputMode::Insert => String::from("i").with(BAR_FOREGROUND),
            InputMode::Normal => String::from("n").with(RED_COLOR),
            InputMode::Visual => String::from("v").with(GREEN_COLOR),
        },
        EMPTY.clone(),
        row_buffer.get_char_count().to_string().with(BAR_FOREGROUND),
        EMPTY.clone(),
        row_buffer.get_buffer_name().to_string().with(PINK_COLOR),
    ]);

    let attribute_len = Attribute::Reset.to_string().len();

    (0..BAR_PADDING).for_each(|_| bar_value.push(' '));
    bar_value.push_str(&bar_left_value);

    // Calculate the adjusted length, considering the ANSI escape codes
    let adjusted_length = size.columns
        - attribute_len // remove the attribute's text (Attribute::Reset) 
        - left_size
        - right_size;

    (0..adjusted_length).for_each(|_| bar_value.push(' '));

    bar_value.push_str(&bar_right_value);
    (0..BAR_PADDING).for_each(|_| bar_value.push(' '));

    // reverse the contents of the bar value
    status_bar.value = wrap_colored(bar_value);
}

/// Builds a part of the status bar content based on the provided content elements.
///
/// # Arguments
///
/// * `content` - A vector of elements that implement the `Into<StyledContent<String>>` trait.
///
/// # Returns
///
/// A tuple containing the total size of the unstyled content, and the concatenated string of styled content.
///
/// # Example
///
/// ```
/// let content = vec![
///     String::from("i").with(BAR_FOREGROUND),
///     EMPTY.clone(),
///     row_buffer.get_char_count().to_string().with(BAR_FOREGROUND),
///     EMPTY.clone(),
///     row_buffer.get_buffer_name().to_string().with(PINK_COLOR),
/// ];
/// let (size, value) = build_bar_part(content);
/// ```
fn build_bar_part(content: Vec<impl Into<StyledContent<String>>>) -> (usize, String) {
    return content
        .into_iter()
        .fold((0, String::new()), |(size_acc, string_acc), content| {
            let styled_content = content.into();

            let content_len = styled_content.content().len();
            let content_str = styled_content.to_string();

            (size_acc + content_len, string_acc + &content_str)
        });
}

fn wrap_colored(value: String) -> String {
    return value.on(BAR_BACKGROUND).to_string() + &Attribute::Reset.to_string();
}

/// A const function that creates a `Color` enum representing an RGB color.
///
/// # Arguments
///
/// * `r` - The red component of the RGB color (0-255).
/// * `g` - The green component of the RGB color (0-255).
/// * `b` - The blue component of the RGB color (0-255).
///
/// # Returns
///
/// A `Color::Rgb` enum variant representing the specified RGB color.
///
/// # Example
///
/// ```
/// const MY_COLOR: Color = color_rgb(255, 0, 0); // Red color
/// ```
pub const fn color_rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb { r, g, b }
}
