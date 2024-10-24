use std::fmt::Display;

pub enum ShieldSprite {
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
impl Display for ShieldSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShieldSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser_shields.png"
            ),
            ShieldSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber_shields.png"
            ),
            ShieldSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought_shields.png"
            ),
            ShieldSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter_shields.png"
            ),
            ShieldSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate_shields.png"
            ),
            ShieldSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout_shields.png"
            ),
            ShieldSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship_shields.png"
            ),
            ShieldSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship_shields.png"
            ),
            ShieldSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser_shields.png"
            ),
            ShieldSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber_shields.png"
            ),
            ShieldSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought_shields.png"
            ),
            ShieldSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter_shields.png"
            ),
            ShieldSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate_shields.png"
            ),
            ShieldSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout_shields.png"
            ),
            ShieldSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship_shields.png"
            ),
            ShieldSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship_shields.png"
            ),
            ShieldSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser_shields.png"
            ),
            ShieldSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber_shields.png"
            ),
            ShieldSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought_shields.png"
            ),
            ShieldSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter_shields.png"
            ),
            ShieldSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate_shields.png"
            ),
            ShieldSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout_shields.png"
            ),
            ShieldSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship_shields.png"
            ),
            ShieldSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship_shields.png"
            ),
            ShieldSprite::GranokBattleCruiser => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_battlecruiser_shields.png"
            ),
            ShieldSprite::GranokBomber => write!(
                formatter,
                "images/factions/granok/starships/bomber/granok_bomber_shields.png"
            ),
            ShieldSprite::GranokDreadnought => write!(
                formatter,
                "images/factions/granok/starships/dreadnought/granok_dreadnought_shields.png"
            ),
            ShieldSprite::GranokFighter => write!(
                formatter,
                "images/factions/granok/starships/fighter/granok_fighter_shields.png"
            ),
            ShieldSprite::GranokFrigate => write!(
                formatter,
                "images/factions/granok/starships/frigate/granok_frigate_shields.png"
            ),
            ShieldSprite::GranokScout => write!(
                formatter,
                "images/factions/granok/starships/scout/granok_scout_shields.png"
            ),
            ShieldSprite::GranokSupportShip => write!(
                formatter,
                "images/factions/granok/starships/support_ship/granok_support_ship_shields.png"
            ),
            ShieldSprite::GranokTorpedoShip => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_shields.png"
            ),
        }
    }
}
