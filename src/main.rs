use bevy_ecs::{
    event::Events,
    schedule::{IntoSystemConfigs, Schedule, SystemSet},
    world::World,
};

use crossterm::terminal;
use input::{data::RowBuffer, reader::InputReader, InputMode, QuitWriter};
use output::{buffer::OutputBuffer, OutputSize};
use std::{
    io::Result,
    panic::{self},
    path::Path,
};

pub mod cursor;
pub mod input;
pub mod output;
pub mod status;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemType {
    Output,
    Input,
    Flush,
    Cursor,
}

fn main() -> Result<()> {
    terminal::enable_raw_mode().expect("Could not enable raw mode");

    let mut world = World::new();
    let mut schedule = Schedule::default();

    // output-systems
    schedule.add_systems(
        (
            // hide the cursor before doing anything
            cursor::hide::dispatch_hide.before(output::clear),
            output::clear,
            output::draw_rows,
            // because the order of systems is not the same as register order, we want to make sure
            // to always run this after draw_rows is called.
            status::draw::draw_status_bar.after(output::draw_rows),
        )
            .in_set(SystemType::Output),
    );

    schedule.add_systems(
        output::flush_buffer
            .after(SystemType::Output)
            .in_set(SystemType::Flush),
    );

    cursor::init(&mut world, &mut schedule);
    status::init(&mut world, &mut schedule);

    // input-systems
    schedule.add_systems(
        (input::process_input, input::quit)
            .after(SystemType::Output)
            .in_set(SystemType::Input),
    );

    world.insert_resource(InputMode::Normal);
    world.insert_resource(InputReader);

    // insert output resources
    world.insert_resource(OutputSize::default());
    world.insert_resource(OutputBuffer::new());

    // hardcoded for now, easier for debugging.
    world.insert_resource(
        RowBuffer::try_from(Path::new(
            "/home/riven/projects/rust/heracross/src/output/mod.rs",
        ))
        .expect("meow"),
    );
    world.insert_resource(Events::<QuitWriter>::default());

    panic::set_hook(Box::new(|_| {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }));

    loop {
        schedule.run(&mut world);
    }
}
