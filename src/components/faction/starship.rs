use crate::{
    assets::images::starship_sprite::{StarshipSprite, StarshipType},
    components::{
        server::server_object::ServerObject, sprite_component::size_component::SizeComponent,
    },
    resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2, prelude::Transform};
use serde::{Deserialize, Serialize};

// TODO spawned starships take damage a radius 1.5 to 2 times the stars size (mechanic)

const SIZE: f32 = TILE_SIZE * 16.0;

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct Starship {
    pub starship_sprite: StarshipSprite,
    pub faction: Faction,
    pub size_component: SizeComponent,
}

impl Starship {
    pub fn new(starship_sprite: StarshipSprite) -> Starship {
        Self {
            starship_sprite,
            faction: Faction::determine_faction(starship_sprite),
            size_component: SizeComponent {
                size: Vec2::new(SIZE, SIZE),
                z_index: 5.0,
            },
        }
    }

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
        let very_fast_speed: f32 = 1000.0;
        let fast_speed: f32 = 500.0;
        let medium_speed: f32 = 350.0;
        let slow_speed: f32 = 175.0;
        let very_slow_speed: f32 = 125.0;

        Self {
            speed: match starship_type {
                StarshipType::Fighter => very_fast_speed,
                StarshipType::TorpedoShip => medium_speed,
                StarshipType::BattleCruiser => slow_speed,
                StarshipType::Dreadnought => slow_speed,
                StarshipType::Corvette => fast_speed,
                StarshipType::Destroyer => medium_speed,
                StarshipType::Battleship => slow_speed,
                StarshipType::IntelShip => very_fast_speed,
                StarshipType::Mothership => very_slow_speed,
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
        let very_small: f32 = 0.5;
        let small: f32 = 0.75;
        let medium: f32 = 1.0;
        let large: f32 = 1.5;
        let very_large: f32 = 3.0;

        Self {
            scale: match starship_type {
                StarshipType::Fighter => very_small,
                StarshipType::TorpedoShip => medium,
                StarshipType::BattleCruiser => large,
                StarshipType::Dreadnought => large,
                StarshipType::Corvette => small,
                StarshipType::Destroyer => medium,
                StarshipType::Battleship => large,
                StarshipType::IntelShip => very_small,
                StarshipType::Mothership => very_large,
                StarshipType::None => 0.0,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableStarship {
    pub starship: Starship,
    pub transform: Transform,
    pub server_object: ServerObject,
}

impl SerializableStarship {
    pub fn new(starship: Starship, transform: Transform, server_object: ServerObject) -> Self {
        Self {
            starship,
            transform,
            server_object,
        }
    }
}

// #[cfg(test)]
// mod starship_speed_should {
//     use super::*;
//     use rstest::rstest;

//     #[rstest]
//     #[case(StarshipType::SupportShip, 350.0)]
//     #[case(StarshipType::Scout, 1000.0)]
//     #[case(StarshipType::Fighter, 500.0)]
//     #[case(StarshipType::TorpedoShip, 350.0)]
//     #[case(StarshipType::Bomber, 350.0)]
//     #[case(StarshipType::Frigate, 175.0)]
//     #[case(StarshipType::BattleCruiser, 175.0)]
//     #[case(StarshipType::Dreadnought, 125.0)]
//     fn new_from_starship_type(#[case] starship_type: StarshipType, #[case] speed: f32) {
//         // When
//         let actual_starship_speed = StarshipSpeed::new_from_starship_type(starship_type);

//         // Then
//         assert_eq!(speed, actual_starship_speed.speed);
//     }
// }
