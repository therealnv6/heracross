use crossterm::{
    event::{KeyCode, KeyModifiers},
    terminal,
};

use crate::output::Output;

use self::reader::InputReader;

pub mod data;
pub mod reader;

pub struct Input {
    reader: InputReader,
    output: Output,
}

impl Input {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not disable raw mode");

        Self {
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

        match (event.code, event.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => return Ok(false),
            (KeyCode::Char(val @ ('h' | 'j' | 'k' | 'l')), KeyModifiers::NONE) => {
                self.output.move_cursor(val);
            }
            _ => {
                println!("{:?}, {:?}", event.code, event.modifiers);
            }
        }

        Ok(true)
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
