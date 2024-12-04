use crate::{
    assets::images::faction_starship_sprite::{
        starship_sprite::StarshipSprite, weapon_sprite::WeaponSprite,
    },
    components::sprite_component::size_component::SizeComponent,
    resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2};
use serde::{Deserialize, Serialize};

const SIZE: f32 = TILE_SIZE;

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct Weapon {
    weapon_sprite: WeaponSprite,
    size: SizeComponent,
    faction: Faction,
}
impl Weapon {
    pub fn new(starship_sprite: StarshipSprite) -> Self {
        let size = SizeComponent {
            size: Vec2::new(SIZE, SIZE),
            z_index: 4.0,
        };

        match starship_sprite {
            StarshipSprite::AtarkBattleCruiser => Self {
                weapon_sprite: WeaponSprite::AtarkBattleCruiser,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkBomber => Self {
                weapon_sprite: WeaponSprite::AtarkBomber,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkDreadnought => Self {
                weapon_sprite: WeaponSprite::AtarkDreadnought,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkFighter => Self {
                weapon_sprite: WeaponSprite::AtarkFighter,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkFrigate => Self {
                weapon_sprite: WeaponSprite::AtarkFrigate,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkScout => Self {
                weapon_sprite: WeaponSprite::AtarkScout,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkSupportShip => Self {
                weapon_sprite: WeaponSprite::AtarkSupportShip,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkTorpedoShip => Self {
                weapon_sprite: WeaponSprite::AtarkTorpedoShip,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::KarcanBattleCruiser => Self {
                weapon_sprite: WeaponSprite::KarcanBattleCruiser,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanBomber => Self {
                weapon_sprite: WeaponSprite::KarcanBomber,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanDreadnought => Self {
                weapon_sprite: WeaponSprite::KarcanDreadnought,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanFighter => Self {
                weapon_sprite: WeaponSprite::KarcanFighter,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanFrigate => Self {
                weapon_sprite: WeaponSprite::KarcanFrigate,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanScout => Self {
                weapon_sprite: WeaponSprite::KarcanScout,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanSupportShip => Self {
                weapon_sprite: WeaponSprite::KarcanSupportShip,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanTorpedoShip => Self {
                weapon_sprite: WeaponSprite::KarcanTorpedoShip,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::NoozlerBattleCruiser => Self {
                weapon_sprite: WeaponSprite::NoozlerBattleCruiser,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerBomber => Self {
                weapon_sprite: WeaponSprite::NoozlerBomber,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerDreadnought => Self {
                weapon_sprite: WeaponSprite::NoozlerDreadnought,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerFighter => Self {
                weapon_sprite: WeaponSprite::NoozlerFighter,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerFrigate => Self {
                weapon_sprite: WeaponSprite::NoozlerFrigate,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerScout => Self {
                weapon_sprite: WeaponSprite::NoozlerScout,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerSupportShip => Self {
                weapon_sprite: WeaponSprite::NoozlerSupportShip,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerTorpedoShip => Self {
                weapon_sprite: WeaponSprite::NoozlerTorpedoShip,
                size,
                faction: Faction::Noozler,
            },
            // TODO Weapon sprites
            StarshipSprite::GranokBattleCruiser => Self {
                weapon_sprite: WeaponSprite::GranokBattleCruiser,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GranokBomber => Self {
                weapon_sprite: WeaponSprite::GranokBomber,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GranokDreadnought => Self {
                weapon_sprite: WeaponSprite::GranokDreadnought,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GranokFighter => Self {
                weapon_sprite: WeaponSprite::GranokFighter,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GranokFrigate => Self {
                weapon_sprite: WeaponSprite::GranokFrigate,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GranokScout => Self {
                weapon_sprite: WeaponSprite::GranokScout,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GranokSupportShip => Self {
                weapon_sprite: WeaponSprite::GranokSupportShip,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GranokTorpedoShip => Self {
                weapon_sprite: WeaponSprite::GranokTorpedoShip,
                size,
                faction: Faction::Noozler,
            },
        }
    }
}
