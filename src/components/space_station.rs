use bevy::{ecs::component::Component, math::Vec2};

use crate::{
    assets::images::space_facility_sprite::SpaceStationSprite, resources::constants::STATION_SIZE,
};

use super::size_component::SizeComponent;

const SPACE_STATION_SIZE: Vec2 = Vec2::new(STATION_SIZE, STATION_SIZE);

#[derive(Component, Clone, Copy)]
pub struct SpaceStation {
    pub sprite_path: SpaceStationSprite,
    pub size_component: SizeComponent,
}

impl SpaceStation {
    pub fn new(sprite_path: SpaceStationSprite) -> Self {
        Self {
            sprite_path,
            size_component: SizeComponent {
                size: SPACE_STATION_SIZE,
                z_index: 3.0,
            },
        }
    }
}
