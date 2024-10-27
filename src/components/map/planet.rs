use crate::{
    assets::images::planet_sprite::PlanetSprite,
    components::sprite_component::size_component::SizeComponent,
    resources::constants::SPACE_TILE_SIZE,
};
use bevy::{ecs::component::Component, math::Vec2};
use rand::Rng;

pub const PLANET_CLOSEST_DISTANCE_TO_SUN: f32 = SIZE + SPACE_TILE_SIZE; // minimum distance
const SIZE: f32 = SPACE_TILE_SIZE * 2.0; // maximum size

#[derive(Component, Clone, Copy)]
pub struct Planet {
    pub sprite_path: PlanetSprite,
    pub size_component: SizeComponent,
}

impl Planet {
    pub fn new(sprite_path: PlanetSprite) -> Self {
        let mut rng = rand::thread_rng();
        let planet_sprite_size: f32 = rng.gen_range(SIZE * 0.25..SIZE);

        Self {
            sprite_path,
            size_component: SizeComponent {
                size: Vec2::new(planet_sprite_size, planet_sprite_size),
                z_index: 2.0,
            },
        }
    }
}
