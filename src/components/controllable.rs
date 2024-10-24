use bevy::{math::Vec3, prelude::Component};

#[derive(Component)]
pub struct Movement {
    pub target_location: Vec3,
    pub max_speed: f32,
    pub current_speed: f32,
}
