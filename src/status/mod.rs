use bevy_ecs::prelude::*;

pub mod draw;
pub mod fill;

pub fn init(world: &mut World, schedule: &mut Schedule) {
    world.insert_resource(StatusBuffer::new());

    // we don't care much about the order of fill_bar, so we'll call it here.
    schedule.add_systems(fill::fill_bar);
}

#[derive(Clone, Resource)]
pub struct StatusBuffer {
    value: String,
}

impl StatusBuffer {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }
}
