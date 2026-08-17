use crate::{
    assets::images::starship_sprite::{StarshipSprite, StarshipType}, components::size_component::SizeComponent, resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2};
use serde::{Deserialize, Serialize};

const SIZE: f32 = TILE_SIZE * 16.0;

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct Starship {
    pub starship_sprite: StarshipSprite,
    pub faction: Faction,
    pub size_component: SizeComponent,
}

impl Starship {
    pub fn new_from_type(starship_selection: StarshipType, faction: Faction) -> Starship {
        let starship_sprite = StarshipSprite::sprite_convert_from(starship_selection, faction);
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
    pub fn new_from_starship_type(starship_type: StarshipType) -> StarshipSpeed {
        let extremely_fast_speed: f32 = 1000.0;
        let very_fast_speed: f32 = 750.0;
        let fast_speed: f32 = 500.0;
        let medium_speed: f32 = 350.0;
        let slow_speed: f32 = 200.0;
        let very_slow_speed: f32 = 150.0;
        let extremely_slow: f32 = 100.0;

        Self {
            speed: match starship_type {
                StarshipType::Fighter => extremely_fast_speed,
                StarshipType::TorpedoShip => very_fast_speed,
                StarshipType::BattleCruiser => slow_speed,
                StarshipType::Dreadnought => very_slow_speed,
                StarshipType::Corvette => fast_speed,
                StarshipType::Destroyer => medium_speed,
                StarshipType::Battleship => slow_speed,
                StarshipType::IntelShip => very_fast_speed,
                StarshipType::Mothership => extremely_slow,
                StarshipType::None => 0.0,
            },
        }
    }
}

pub struct StarshipSize {
    pub scale: f32,
}

impl StarshipSize {
    pub fn new_from_starship_type(starship_type: StarshipType) -> StarshipSize {
        let very_small: f32 = 0.4;
        let small: f32 = 0.5;
        let medium: f32 = 1.0;
        let large: f32 = 1.5;
        let very_large: f32 = 2.0;
        let extremely_large: f32 = 3.0;

        Self {
            scale: match starship_type {
                StarshipType::Fighter => very_small,
                StarshipType::TorpedoShip => small,
                StarshipType::BattleCruiser => large,
                StarshipType::Dreadnought => large,
                StarshipType::Corvette => small,
                StarshipType::Destroyer => medium,
                StarshipType::Battleship => very_large,
                StarshipType::IntelShip => very_small,
                StarshipType::Mothership => extremely_large,
                StarshipType::None => 0.0,
            },
        }
    }
}

// #[derive(Serialize, Deserialize)]
// pub struct SerializableStarship {
//     pub starship: Starship,
//     pub transform: Transform,
//     pub server_object: ServerObject,
// }

// impl SerializableStarship {
//     pub fn new(starship: Starship, transform: Transform, server_object: ServerObject) -> Self {
//         Self {
//             starship,
//             transform,
//             server_object,
//         }
//     }
// }
