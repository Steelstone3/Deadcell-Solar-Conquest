use bevy::{ecs::message::Message, math::Vec2};

#[derive(Message)]
pub struct MouseLeftClickEvent {
    pub cursor_world_position: Vec2,
}

#[derive(Message)]
pub struct MouseLeftClickModifierEvent {
    pub cursor_world_position: Vec2,
    pub just_released: bool,
}

#[derive(Message)]
pub struct MouseRightClickEvent {
    pub cursor_world_position: Vec2,
}
