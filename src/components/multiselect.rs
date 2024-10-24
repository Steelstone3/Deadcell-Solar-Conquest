use bevy::{math::Vec2, prelude::Component};

#[derive(Component, Clone, Copy)]
pub struct MultiSelect {
    pub start: Vec2,
    pub end: Vec2,
}
