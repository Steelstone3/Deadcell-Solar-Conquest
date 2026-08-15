use crate::{
    assets::images::starship_sprite::StarbaseSprite,
    components::sprite_component::size_component::SizeComponent,
    resources::constants::SPACE_TILE_SIZE,
};
use bevy::{ecs::component::Component, math::Vec2, prelude::Transform};
use serde::{Deserialize, Serialize};

const SIZE: f32 = SPACE_TILE_SIZE * 1.5;
const SPACE_FACILITY_SIZE: Vec2 = Vec2::new(SIZE, SIZE);

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Starbase {
    pub sprite_path: StarbaseSprite,
    pub size_component: SizeComponent,
}

impl Starbase {
    pub fn new(space_facility_sprite: StarbaseSprite) -> Starbase {
        Self {
            sprite_path: space_facility_sprite,
            size_component: SizeComponent {
                size: SPACE_FACILITY_SIZE,
                z_index: 3.0,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableSpaceFacility {
    pub space_facility: Starbase,
    pub transform: Transform,
}

impl SerializableSpaceFacility {
    pub fn new(space_facility: Starbase, transform: Transform) -> Self {
        Self {
            space_facility,
            transform,
        }
    }
}
