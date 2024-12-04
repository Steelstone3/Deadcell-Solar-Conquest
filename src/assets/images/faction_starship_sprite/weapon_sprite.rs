use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum WeaponSprite {
    AtarkBattleCruiser,
    AtarkBomber,
    AtarkDreadnought,
    AtarkFighter,
    AtarkFrigate,
    AtarkScout,
    AtarkSupportShip,
    AtarkTorpedoShip,
    KarcanBattleCruiser,
    KarcanBomber,
    KarcanDreadnought,
    KarcanFighter,
    KarcanFrigate,
    KarcanScout,
    KarcanSupportShip,
    KarcanTorpedoShip,
    NoozlerBattleCruiser,
    NoozlerBomber,
    NoozlerDreadnought,
    NoozlerFighter,
    NoozlerFrigate,
    NoozlerScout,
    NoozlerSupportShip,
    NoozlerTorpedoShip,
    GranokBattleCruiser,
    GranokBomber,
    GranokDreadnought,
    GranokFighter,
    GranokFrigate,
    GranokScout,
    GranokSupportShip,
    GranokTorpedoShip,
}

// TODO weapon sprites
impl Display for WeaponSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponSprite::AtarkBattleCruiser => write!(formatter, ""),
            WeaponSprite::AtarkBomber => write!(formatter, ""),
            WeaponSprite::AtarkDreadnought => write!(formatter, ""),
            WeaponSprite::AtarkFighter => write!(formatter, ""),
            WeaponSprite::AtarkFrigate => write!(formatter, ""),
            WeaponSprite::AtarkScout => write!(formatter, ""),
            WeaponSprite::AtarkSupportShip => write!(formatter, ""),
            WeaponSprite::AtarkTorpedoShip => write!(formatter, ""),
            WeaponSprite::KarcanBattleCruiser => write!(formatter, ""),
            WeaponSprite::KarcanBomber => write!(formatter, ""),
            WeaponSprite::KarcanDreadnought => write!(formatter, ""),
            WeaponSprite::KarcanFighter => write!(formatter, ""),
            WeaponSprite::KarcanFrigate => write!(formatter, ""),
            WeaponSprite::KarcanScout => write!(formatter, ""),
            WeaponSprite::KarcanSupportShip => write!(formatter, ""),
            WeaponSprite::KarcanTorpedoShip => write!(formatter, ""),
            WeaponSprite::NoozlerBattleCruiser => write!(formatter, ""),
            WeaponSprite::NoozlerBomber => write!(formatter, ""),
            WeaponSprite::NoozlerDreadnought => write!(formatter, ""),
            WeaponSprite::NoozlerFighter => write!(formatter, ""),
            WeaponSprite::NoozlerFrigate => write!(formatter, ""),
            WeaponSprite::NoozlerScout => write!(formatter, ""),
            WeaponSprite::NoozlerSupportShip => write!(formatter, ""),
            WeaponSprite::NoozlerTorpedoShip => write!(formatter, ""),
            WeaponSprite::GranokBattleCruiser => write!(formatter, ""),
            WeaponSprite::GranokBomber => write!(formatter, ""),
            WeaponSprite::GranokDreadnought => write!(formatter, ""),
            WeaponSprite::GranokFighter => write!(formatter, ""),
            WeaponSprite::GranokFrigate => write!(formatter, ""),
            WeaponSprite::GranokScout => write!(formatter, ""),
            WeaponSprite::GranokSupportShip => write!(formatter, ""),
            WeaponSprite::GranokTorpedoShip => write!(formatter, ""),
        }
    }
}
