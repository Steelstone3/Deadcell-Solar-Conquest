use bevy::{ecs::event::Event, math::Vec2};

#[derive(Event)]
pub struct MouseRightClickEvent {
    pub cursor_world_position: Vec2,
}
