use super::{
    size_component::SizeComponent, starship_sprite_bundle::StarshipSpriteBundle, weapon::Weapon,
};
use crate::{
    assets::{
        images::{
            faction_starship_sprite::starship_sprite::StarshipSprite, starship_type::StarshipType,
        },
        user_interface::icons::starship_icons::StarshipIcon,
    },
    resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2};

// TODO spawned starships take damage a radius 1.5 to 2 times the suns size (mechanic)

const SIZE: f32 = TILE_SIZE;

const FAST_SHIP_SPEED: f32 = 100.0;
const MEDIUM_SHIP_SPEED: f32 = 75.0;
const SLOW_SHIP_SPEED: f32 = 50.0;

#[derive(Component)]
pub struct Starship {
    pub starship_sprite_bundle: StarshipSpriteBundle,
    #[allow(dead_code)]
    pub weapon: Weapon,
    #[allow(dead_code)]
    pub faction: Faction,
    pub size_component: SizeComponent,
}

impl Starship {
    pub fn new(starship_sprite: StarshipSprite) -> Starship {
        Self {
            starship_sprite_bundle: StarshipSpriteBundle::new(starship_sprite),
            faction: Faction::determine_faction(starship_sprite),
            size_component: SizeComponent {
                size: Vec2::new(SIZE, SIZE),
                z_index: 5.0,
            },
            weapon: Weapon::new(starship_sprite),
        }
    }

    pub fn new_from_icon(starship_icon: StarshipIcon) -> Starship {
        let starship_sprite = StarshipSprite::sprite_convert_from(starship_icon);

        Self {
            starship_sprite_bundle: StarshipSpriteBundle::new(starship_sprite),
            faction: Faction::determine_faction(starship_sprite),
            size_component: SizeComponent {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                z_index: 5.0,
            },
            weapon: Weapon::new(starship_sprite),
        }
    }
}

pub struct ShipSpeed {
    pub speed: f32,
}

impl ShipSpeed {
    pub fn new_from_ship_type(ship_type: StarshipType) -> ShipSpeed {
        Self {
            speed: match ship_type {
                StarshipType::SupportShip => MEDIUM_SHIP_SPEED,
                StarshipType::Scout => FAST_SHIP_SPEED,
                StarshipType::Fighter => FAST_SHIP_SPEED,
                StarshipType::TorpedoShip => SLOW_SHIP_SPEED,
                StarshipType::Bomber => MEDIUM_SHIP_SPEED,
                StarshipType::Frigate => SLOW_SHIP_SPEED,
                StarshipType::BattleCruiser => MEDIUM_SHIP_SPEED,
                StarshipType::Dreadnought => SLOW_SHIP_SPEED,
            },
        }
    }
}
