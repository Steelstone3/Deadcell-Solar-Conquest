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

const SIZE: f32 = TILE_SIZE * 16.0;

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
                size: Vec2::new(SIZE, SIZE),
                z_index: 5.0,
            },
            weapon: Weapon::new(starship_sprite),
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
                StarshipType::SupportShip => medium_speed,
                StarshipType::Scout => very_fast_speed,
                StarshipType::Fighter => fast_speed,
                StarshipType::TorpedoShip => medium_speed,
                StarshipType::Bomber => medium_speed,
                StarshipType::Frigate => slow_speed,
                StarshipType::BattleCruiser => slow_speed,
                StarshipType::Dreadnought => very_slow_speed,
            },
        }
    }
}

#[cfg(test)]
mod starship_speed_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(StarshipType::SupportShip, 350.0)]
    #[case(StarshipType::Scout, 1000.0)]
    #[case(StarshipType::Fighter, 500.0)]
    #[case(StarshipType::TorpedoShip, 350.0)]
    #[case(StarshipType::Bomber, 350.0)]
    #[case(StarshipType::Frigate, 175.0)]
    #[case(StarshipType::BattleCruiser, 175.0)]
    #[case(StarshipType::Dreadnought, 125.0)]
    fn new_from_starship_type(#[case] starship_type: StarshipType, #[case] speed: f32) {
        // When
        let actual_starship_speed = StarshipSpeed::new_from_starship_type(starship_type);

        // Then
        assert_eq!(speed, actual_starship_speed.speed);
    }
}
