use bevy::math::Vec2;

pub const TILE_SIZE: f32 = 32.0;
pub const SPACE_TILE_SIZE: f32 = 32.0 * TILE_SIZE;
pub const SPACE_SIZE: Vec2 = Vec2::new(SPACE_TILE_SIZE, SPACE_TILE_SIZE);