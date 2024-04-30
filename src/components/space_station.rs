use bevy::{ecs::component::Component, math::Vec2};

use crate::{
    assets::images::space_stations::SpaceStationSprite, resources::constants::STATION_SIZE,
};

const SPACE_STATION_SIZE: Vec2 = Vec2::new(STATION_SIZE, STATION_SIZE);

#[derive(Component, Clone, Copy)]
pub struct SpaceStation {
    pub sprite_path: SpaceStationSprite,
    pub size: Vec2,
    pub z_index: f32,
}

impl SpaceStation {
    pub fn new(sprite_path: SpaceStationSprite) -> Self {
        Self {
            sprite_path,
            size: SPACE_STATION_SIZE,
            z_index: 1.0,
        }
    }
}
