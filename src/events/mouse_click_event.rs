use bevy::{ecs::event::Event, math::Vec2};

#[derive(Event)]
pub struct MouseClickEvent {
    pub ctrl_modifier: bool,
    pub cursor_world_position: Vec2,
    pub just_released: bool,
}
