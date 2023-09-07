use crate::SystemType;
use bevy_ecs::prelude::*;

pub mod hide;
pub mod scroll;
pub mod update;

pub fn init(world: &mut World, schedule: &mut Schedule) {
    world.insert_resource(CursorPosition::default());
    world.insert_resource(CursorOffset::default());
    world.insert_resource(Events::<CursorHideEvent>::default());
    world.insert_resource(Events::<CursorMoveEvent>::default());

    schedule.add_systems(
        (
            update::handle_cursor_move,
            hide::handle_cursor_hide,
            update::update_cursor,
            scroll::scroll,
        )
            .in_set(SystemType::Cursor),
    );
}

#[derive(Resource, Default)]
pub struct CursorPosition {
    x: usize,
    y: usize,
}

#[derive(Clone, Resource, Default)]
pub struct CursorOffset {
    pub(crate) row: usize,
    pub(crate) column: usize,
    pub(crate) render: usize,
}

#[derive(Event)]
pub struct CursorMoveEvent {
    direction: CursorDirection,
}

#[derive(Event, Default)]
pub struct CursorHideEvent;

pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for CursorMoveEvent {
    fn from(value: char) -> Self {
        let direction = match value {
            'h' => CursorDirection::Left,
            'j' => CursorDirection::Down,
            'k' => CursorDirection::Up,
            'l' => CursorDirection::Right,
            _ => unimplemented!(),
        };

        Self { direction }
    }
}
