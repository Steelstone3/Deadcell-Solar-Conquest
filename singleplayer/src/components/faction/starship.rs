use crate::{
    assets::sprites::starship_sprites::StarshipSprites,
    components::{size_component::SizeComponent, types::starship_types::StarshipTypes},
    resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2};
use serde::{Deserialize, Serialize};

const SIZE: f32 = TILE_SIZE * 16.0;

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct Starship {
    pub starship_sprite: StarshipSprites,
    pub faction: Faction,
    pub size_component: SizeComponent,
}

impl Starship {
    pub fn new_from_type(starship_selection: StarshipTypes, faction: Faction) -> Starship {
        let starship_sprite = StarshipSprites::sprite_convert_from(starship_selection, faction);
        let starship_size = StarshipSize::new_from_starship_type(starship_selection);

        Self {
            starship_sprite,
            faction: Faction::determine_faction(starship_sprite),
            size_component: SizeComponent {
                size: Vec2::new(SIZE * starship_size.scale, SIZE * starship_size.scale),
                z_index: 5.0,
            },
        }
    }
}

pub struct StarshipSpeed {
    pub speed: f32,
}

impl StarshipSpeed {
    pub fn new_from_starship_type(starship_type: StarshipTypes) -> StarshipSpeed {
        let extremely_fast_speed: f32 = 1000.0;
        let very_fast_speed: f32 = 750.0;
        let fast_speed: f32 = 500.0;
        let medium_speed: f32 = 350.0;
        let slow_speed: f32 = 200.0;
        let very_slow_speed: f32 = 150.0;
        let extremely_slow: f32 = 100.0;

        Self {
            speed: match starship_type {
                StarshipTypes::Fighter => extremely_fast_speed,
                StarshipTypes::TorpedoShip => very_fast_speed,
                StarshipTypes::BattleCruiser => slow_speed,
                StarshipTypes::Dreadnought => very_slow_speed,
                StarshipTypes::Corvette => fast_speed,
                StarshipTypes::Destroyer => medium_speed,
                StarshipTypes::Battleship => slow_speed,
                StarshipTypes::IntelShip => very_fast_speed,
                StarshipTypes::Mothership => extremely_slow,
                StarshipTypes::None => 0.0,
            },
        }
    }
}

pub struct StarshipSize {
    pub scale: f32,
}

impl StarshipSize {
    pub fn new_from_starship_type(starship_type: StarshipTypes) -> StarshipSize {
        let very_small: f32 = 0.4;
        let small: f32 = 0.5;
        let medium: f32 = 1.0;
        let large: f32 = 1.5;
        let very_large: f32 = 2.0;
        let extremely_large: f32 = 3.0;

        Self {
            scale: match starship_type {
                StarshipTypes::Fighter => very_small,
                StarshipTypes::TorpedoShip => small,
                StarshipTypes::BattleCruiser => large,
                StarshipTypes::Dreadnought => large,
                StarshipTypes::Corvette => small,
                StarshipTypes::Destroyer => medium,
                StarshipTypes::Battleship => very_large,
                StarshipTypes::IntelShip => very_small,
                StarshipTypes::Mothership => extremely_large,
                StarshipTypes::None => 0.0,
            },
        }
    }
}
