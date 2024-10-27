use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Clone, Copy)]
pub struct SizeComponent {
    pub size: Vec2,
    pub z_index: f32,
}
