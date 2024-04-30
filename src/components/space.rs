use crate::{assets::images::space::SpaceSprite, resources::constants::TILE_SIZE};
use bevy::{ecs::component::Component, math::Vec2};

const SPACE_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Space {
    pub sprite_path: SpaceSprite,
    pub size: Vec2,
    pub z_index: f32,
}

impl Space {
    pub fn new(sprite_path: SpaceSprite) -> Self {
        Self {
            sprite_path,
            size: SPACE_SIZE,
            z_index: 0.0,
        }
    }
}
