use std::fmt::Display;

pub enum EngineSprite {
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
impl Display for EngineSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser_engines.png"
            ),
            EngineSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber_engines.png"
            ),
            EngineSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought_engines.png"
            ),
            EngineSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter_engines.png"
            ),
            EngineSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate_engines.png"
            ),
            EngineSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout_engines.png"
            ),
            EngineSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship_engines.png"
            ),
            EngineSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship_engines.png"
            ),
            EngineSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser_engines.png"
            ),
            EngineSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber_engines.png"
            ),
            EngineSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought_engines.png"
            ),
            EngineSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter_engines.png"
            ),
            EngineSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate_engines.png"
            ),
            EngineSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout_engines.png"
            ),
            EngineSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship_engines.png"
            ),
            EngineSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship_engines.png"
            ),
            EngineSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser_engines.png"
            ),
            EngineSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber_engines.png"
            ),
            EngineSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought_engines.png"
            ),
            EngineSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter_engines.png"
            ),
            EngineSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate_engines.png"
            ),
            EngineSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout_engines.png"
            ),
            EngineSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship_engines.png"
            ),
            EngineSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokBattleCruiser => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokBomber => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokDreadnought => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokFighter => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokFrigate => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokScout => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokSupportShip => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
            EngineSprite::GranokTorpedoShip => write!(
                formatter,
                "images/factions/granok/starships/battlecruiser/granok_torpedo_ship_engines.png"
            ),
        }
    }
}
