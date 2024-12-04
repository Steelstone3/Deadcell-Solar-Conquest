use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum DestructionSprite {
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

impl Display for DestructionSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DestructionSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser_destruction.png"
            ),
            DestructionSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber_destruction.png"
            ),
            DestructionSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought_destruction.png"
            ),
            DestructionSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter_destruction.png"
            ),
            DestructionSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate_destruction.png"
            ),
            DestructionSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout_destruction.png"
            ),
            DestructionSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship_destruction.png"
            ),
            DestructionSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship_destruction.png"
            ),
            DestructionSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser_destruction.png"
            ),
            DestructionSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber_destruction.png"
            ),
            DestructionSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought_destruction.png"
            ),
            DestructionSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter_destruction.png"
            ),
            DestructionSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate_destruction.png"
            ),
            DestructionSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout_destruction.png"
            ),
            DestructionSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship_destruction.png"
            ),
            DestructionSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship_destruction.png"
            ),
            DestructionSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser_destruction.png"
            ),
            DestructionSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber_destruction.png"
            ),
            DestructionSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought_destruction.png"
            ),
            DestructionSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter_destruction.png"
            ),
            DestructionSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate_destruction.png"
            ),
            DestructionSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout_destruction.png"
            ),
            DestructionSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship_destruction.png"
            ),
            DestructionSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokBattleCruiser => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokBomber => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokDreadnought => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokFighter => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokFrigate => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokScout => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokSupportShip => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
            DestructionSprite::GranokTorpedoShip => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_destruction.png"
            ),
        }
    }
}
