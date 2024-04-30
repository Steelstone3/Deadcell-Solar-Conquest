use crate::{assets::images::planets::PlanetSprite, resources::constants::PLANET_TILE_SIZE};
use bevy::{ecs::component::Component, math::Vec2};
use rand::Rng;

#[derive(Component, Clone, Copy)]
pub struct Planet {
    pub sprite_path: PlanetSprite,
    pub size: Vec2,
    pub z_index: f32,
}

impl Planet {
    pub fn new(sprite_path: PlanetSprite) -> Self {
        let mut rng = rand::thread_rng();
        let planet_sprite_size: f32 = rng.gen_range(PLANET_TILE_SIZE * 0.25..PLANET_TILE_SIZE);

        Self {
            sprite_path,
            size: Vec2::new(planet_sprite_size, planet_sprite_size),
            z_index: 0.0,
        }
    }
}
