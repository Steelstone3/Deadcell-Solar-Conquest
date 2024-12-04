use crate::{
    assets::images::space_sprite::SpaceSprite,
    components::sprite_component::size_component::SizeComponent,
    resources::constants::SPACE_TILE_SIZE,
};
use bevy::{ecs::component::Component, math::Vec2, prelude::Transform};
use serde::{Deserialize, Serialize};

const SPACE_SIZE: Vec2 = Vec2::new(SPACE_TILE_SIZE, SPACE_TILE_SIZE);

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Space {
    pub sprite_path: SpaceSprite,
    pub size_component: SizeComponent,
}

impl Space {
    pub fn new(sprite_path: SpaceSprite) -> Self {
        Self {
            sprite_path,
            size_component: SizeComponent {
                size: SPACE_SIZE,
                z_index: 0.0,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableSpace {
    pub space: Space,
    pub transform: Transform,
}

impl SerializableSpace {
    pub fn new(space: Space, transform: Transform) -> Self {
        Self { space, transform }
    }
}
