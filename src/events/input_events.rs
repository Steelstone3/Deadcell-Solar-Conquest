use bevy::{ecs::event::Event, math::Vec2};

#[derive(Event)]
pub struct MouseLeftClickEvent {
    pub cursor_world_position: Vec2,
}

#[derive(Event)]
pub struct MouseLeftClickModifierEvent {
    pub cursor_world_position: Vec2,
    pub just_released: bool,
}

#[derive(Event)]
pub struct MouseRightClickEvent {
    pub cursor_world_position: Vec2,
}
