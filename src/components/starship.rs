use super::{
    size_component::SizeComponent, starship_sprite_bundle::StarshipSpriteBundle, weapon::Weapon,
};
use crate::{
    assets::images::faction_starships::starships::StarshipSprite,
    resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
#[allow(dead_code)]
pub struct Starship {
    pub starship_sprite_bundle: StarshipSpriteBundle,
    pub weapon: Weapon,
    pub faction: Faction,
    pub size_component: SizeComponent,
}

impl Starship {
    #[allow(dead_code)]
    pub fn new(starship_sprite: StarshipSprite) -> Starship {
        Self {
            starship_sprite_bundle: StarshipSpriteBundle::new(starship_sprite),
            faction: Faction::new(starship_sprite),
            size_component: SizeComponent {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                z_index: 5.0,
            },
            weapon: Weapon::new(starship_sprite),
        }
    }
}
