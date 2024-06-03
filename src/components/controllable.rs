use bevy::{math::Vec3, prelude::Component};

#[derive(Component)]
pub struct Movement {
    pub target_location: Vec3,
}
