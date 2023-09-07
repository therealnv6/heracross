use std::time::Duration;

use bevy_ecs::system::Resource;
use crossterm::event::{self, Event, KeyEvent};

#[derive(Resource)]
pub struct InputReader;

impl InputReader {
    pub fn read_key(&self) -> Result<KeyEvent, std::io::Error> {
        loop {
            if event::poll(Duration::from_millis(150))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}
