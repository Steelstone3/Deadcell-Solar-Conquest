use bevy::{ecs::component::Component, math::Vec2};
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct SizeComponent {
    pub size: Vec2,
    pub z_index: f32,
}
