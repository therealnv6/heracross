use bevy_ecs::{
    schedule::{IntoSystemConfigs, Schedule},
    world::World,
};
use input::{Input, InputMode};
use std::io::Result;

pub mod input;
pub mod output;

fn main() -> Result<()> {
    let mut world = World::new();
    let mut schedule = Schedule::default();

    schedule.add_systems((
        output::status::fill_bar,
        output::draw_rows.after(output::status::fill_bar),
        output::flush_buffer.after(output::draw_rows),
    ));

    world.insert_resource(InputMode::Normal);
    let mut input = Input::new();

    while input.run()? {}

    Ok(())
}
