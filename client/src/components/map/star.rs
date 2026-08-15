use bevy::{ecs::component::Component, math::Vec2, transform::components::Transform};
use serde::{Deserialize, Serialize};

use crate::{
    assets::images::star_sprite::StarSprite,
    components::sprite_component::size_component::SizeComponent,
    resources::constants::SPACE_TILE_SIZE,
};

const SIZE: f32 = SPACE_TILE_SIZE * 4.0; // minimum size
const STAR_SIZE: Vec2 = Vec2::new(SIZE, SIZE);

#[derive(Component, Clone, Copy, Deserialize, Serialize)]
pub struct Star {
    pub sprite_path: StarSprite,
    pub size_component: SizeComponent,
}

impl Star {
    pub fn new(sprite_path: StarSprite) -> Self {
        Self {
            sprite_path,
            size_component: SizeComponent {
                size: STAR_SIZE,
                z_index: 1.0,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableStar {
    pub star: Star,
    pub transform: Transform,
}

impl SerializableStar {
    pub fn new(star: Star, transform: Transform) -> Self {
        Self { star, transform }
    }
}
