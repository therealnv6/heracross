use input::Input;
use std::io::Result;

pub mod input;
pub mod output;

fn main() -> Result<()> {
    let mut input = Input::new();

    while input.run()? {}

    Ok(())
}
