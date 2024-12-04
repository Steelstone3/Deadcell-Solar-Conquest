use bevy::{math::Vec3, prelude::Component};
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct Movement {
    pub target_location: Vec3,
    pub maximum_speed: f32,
    pub current_speed: f32,
}
