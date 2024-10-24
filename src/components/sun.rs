use bevy::{ecs::component::Component, math::Vec2};

use crate::{assets::images::sun_sprite::SunSprite, resources::constants::SPACE_TILE_SIZE};

use super::size_component::SizeComponent;

const SIZE: f32 = SPACE_TILE_SIZE; // minimum size
const SUN_SIZE: Vec2 = Vec2::new(SIZE, SIZE);

#[derive(Component, Clone, Copy)]
pub struct Sun {
    pub sprite_path: SunSprite,
    pub size_component: SizeComponent,
}

impl Sun {
    pub fn new(sprite_path: SunSprite) -> Self {
        Self {
            sprite_path,
            size_component: SizeComponent {
                size: SUN_SIZE,
                z_index: 1.0,
            },
        }
    }
}
