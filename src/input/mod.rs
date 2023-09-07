use std::fmt::Display;

use bevy_ecs::system::Resource;
use crossterm::{
    event::{KeyCode, KeyModifiers},
    terminal,
};

use crate::output::Output;

use self::reader::InputReader;

pub mod data;
pub mod reader;

#[derive(Copy, Clone, Resource)]
pub enum InputMode {
    Insert,
    Visual,
    Normal,
}

pub struct Input {
    reader: InputReader,
    mode: InputMode,
    output: Output,
}

impl Input {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not disable raw mode");

        Self {
            mode: InputMode::Normal,
            reader: InputReader,
            output: Output::new(),
        }
    }

    pub fn run(&mut self) -> Result<bool, std::io::Error> {
        self.output.refresh()?;
        self.process_input()
    }

    fn process_input(&mut self) -> Result<bool, std::io::Error> {
        let event = self.reader.read_key()?;

        match (event.code, event.modifiers, self.mode) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL, InputMode::Normal) => return Ok(false),
            (KeyCode::Char('i'), KeyModifiers::NONE, InputMode::Normal) => {
                self.mode = InputMode::Insert;
            }
            (KeyCode::Esc, KeyModifiers::NONE, _) => {
                self.mode = InputMode::Normal;
            }
            (
                KeyCode::Char(val @ ('h' | 'j' | 'k' | 'l')),
                KeyModifiers::NONE,
                // only move when in either normal or input mode
                InputMode::Visual | InputMode::Normal,
            ) => {
                self.output.move_cursor(val);
            }
            _ => {
                println!("{:?}, {:?}", event.code, event.modifiers);
            }
        }

        Ok(true)
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

impl Drop for Input {
    fn drop(&mut self) {
        self.output
            .clear()
            .expect("Couldn't clear screen, did something go wrong during shutdown?");

        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}
