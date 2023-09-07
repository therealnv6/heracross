use std::fmt::Display;

use bevy_ecs::{
    event::{Event, EventReader, EventWriter},
    system::{ResMut, Resource},
};
use crossterm::event::{KeyCode, KeyModifiers};

use crate::cursor::CursorMoveEvent;

use self::reader::InputReader;

pub mod data;
pub mod reader;

#[derive(Copy, Clone, Resource)]
pub enum InputMode {
    Insert,
    Visual,
    Normal,
}

#[derive(Event, Default)]
pub struct QuitWriter;

pub fn process_input(
    reader: ResMut<InputReader>,
    mut quit_writer: EventWriter<QuitWriter>,
    mut cursor_move_writer: EventWriter<CursorMoveEvent>,
    mut input_mode: ResMut<InputMode>,
) {
    let event = reader.read_key().expect("Could not read input!");

    match (event.code, event.modifiers, *input_mode) {
        (KeyCode::Char('q'), KeyModifiers::CONTROL, InputMode::Normal) => {
            quit_writer.send_default();
            return;
        }
        (KeyCode::Char('i'), KeyModifiers::NONE, InputMode::Normal) => {
            *input_mode = InputMode::Insert;
        }
        (KeyCode::Esc, KeyModifiers::NONE, _) => {
            *input_mode = InputMode::Normal;
        }
        (
            KeyCode::Char(val @ ('h' | 'j' | 'k' | 'l')),
            KeyModifiers::NONE,
            // only move when in either normal or input mode
            InputMode::Visual | InputMode::Normal,
        ) => cursor_move_writer.send(CursorMoveEvent::from(val)),
        _ => {
            println!("{:?}, {:?}", event.code, event.modifiers);
        }
    }
}

pub fn quit(mut events: EventReader<QuitWriter>) {
    for _ in events.iter() {
        panic!();
    }
}

impl Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InputMode::Normal => "[normal]",
                InputMode::Insert => "[insert]",
                InputMode::Visual => "[visual]",
            }
        )
    }
}
