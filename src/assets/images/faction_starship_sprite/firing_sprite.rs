use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum FiringSprite {
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

// TODO asset paths
impl Display for FiringSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FiringSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser_firing.png"
            ),
            FiringSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber_firing.png"
            ),
            FiringSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought_firing.png"
            ),
            FiringSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter_firing.png"
            ),
            FiringSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate_firing.png"
            ),
            FiringSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout_firing.png"
            ),
            FiringSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship_firing.png"
            ),
            FiringSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship_firing.png"
            ),
            FiringSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser_firing.png"
            ),
            FiringSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber_firing.png"
            ),
            FiringSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought_firing.png"
            ),
            FiringSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter_firing.png"
            ),
            FiringSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate_firing.png"
            ),
            FiringSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout_firing.png"
            ),
            FiringSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship_firing.png"
            ),
            FiringSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship_firing.png"
            ),
            FiringSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser_firing.png"
            ),
            FiringSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber_firing.png"
            ),
            FiringSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought_firing.png"
            ),
            FiringSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter_firing.png"
            ),
            FiringSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate_firing.png"
            ),
            FiringSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout_firing.png"
            ),
            FiringSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship_firing.png"
            ),
            FiringSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokBattleCruiser => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokBomber => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokDreadnought => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokFighter => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokFrigate => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokScout => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokSupportShip => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
            FiringSprite::GranokTorpedoShip => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_firing.png"
            ),
        }
    }
}
