use super::{
    size_component::SizeComponent, starship_sprite_bundle::StarshipSpriteBundle, weapon::Weapon,
};
use crate::{
    assets::{
        images::faction_starship_sprite::starship_sprite::StarshipSprite,
        user_interace::icons::starship_icons::StarshipIcon,
    },
    resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2};

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
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
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
