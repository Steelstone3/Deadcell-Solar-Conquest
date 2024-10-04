use std::fmt::Display;

use crate::assets::user_interace::icons::starship_icons::StarshipIcon;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum StarshipSprite {
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
}

impl Display for StarshipSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser.png"
            ),
            StarshipSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/bomber/atark_bomber.png"
            ),
            StarshipSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/dreadnought/atark_dreadnought.png"
            ),
            StarshipSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/fighter/atark_fighter.png"
            ),
            StarshipSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/frigate/atark_frigate.png"
            ),
            StarshipSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/scout/atark_scout.png"
            ),
            StarshipSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/support_ship/atark_support_ship.png"
            ),
            StarshipSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/torpedo_ship/atark_torpedo_ship.png"
            ),
            StarshipSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser.png"
            ),
            StarshipSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/bomber/karcan_bomber.png"
            ),
            StarshipSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/dreadnought/karcan_dreadnought.png"
            ),
            StarshipSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/fighter/karcan_fighter.png"
            ),
            StarshipSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/frigate/karcan_frigate.png"
            ),
            StarshipSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/scout/karcan_scout.png"
            ),
            StarshipSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/support_ship/karcan_support_ship.png"
            ),
            StarshipSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/torpedo_ship/karcan_torpedo_ship.png"
            ),
            StarshipSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser.png"
            ),
            StarshipSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/bomber/noozler_bomber.png"
            ),
            StarshipSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/dreadnought/noozler_dreadnought.png"
            ),
            StarshipSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/fighter/noozler_fighter.png"
            ),
            StarshipSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/frigate/noozler_frigate.png"
            ),
            StarshipSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/scout/noozler_scout.png"
            ),
            StarshipSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/support_ship/noozler_support_ship.png"
            ),
            StarshipSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/torpedo_ship/noozler_torpedo_ship.png"
            ),
        }
    }
}

impl StarshipSprite {
    pub fn convert_from(starship_icon: StarshipIcon) -> StarshipSprite {
        match starship_icon {
            StarshipIcon::AtarkSupportShip => StarshipSprite::AtarkSupportShip,
            StarshipIcon::AtarkScout => StarshipSprite::AtarkScout,
            StarshipIcon::AtarkFighter => StarshipSprite::AtarkFighter,
            StarshipIcon::AtarkTorpedoShip => StarshipSprite::AtarkTorpedoShip,
            StarshipIcon::AtarkBomber => StarshipSprite::AtarkBomber,
            StarshipIcon::AtarkFrigate => StarshipSprite::AtarkFrigate,
            StarshipIcon::AtarkBattleCruiser => StarshipSprite::AtarkBattleCruiser,
            StarshipIcon::AtarkDreadnought => StarshipSprite::AtarkDreadnought,
            StarshipIcon::KarcanSupportShip => StarshipSprite::KarcanSupportShip,
            StarshipIcon::KarcanScout => StarshipSprite::KarcanScout,
            StarshipIcon::KarcanFighter => StarshipSprite::KarcanFighter,
            StarshipIcon::KarcanTorpedoShip => StarshipSprite::KarcanTorpedoShip,
            StarshipIcon::KarcanBomber => StarshipSprite::KarcanBomber,
            StarshipIcon::KarcanFrigate => StarshipSprite::KarcanFrigate,
            StarshipIcon::KarcanBattleCruiser => StarshipSprite::KarcanBattleCruiser,
            StarshipIcon::KarcanDreadnought => StarshipSprite::KarcanDreadnought,
            StarshipIcon::NoozlerSupportShip => StarshipSprite::NoozlerSupportShip,
            StarshipIcon::NoozlerScout => StarshipSprite::NoozlerScout,
            StarshipIcon::NoozlerFighter => StarshipSprite::NoozlerFighter,
            StarshipIcon::NoozlerTorpedoShip => StarshipSprite::NoozlerTorpedoShip,
            StarshipIcon::NoozlerBomber => StarshipSprite::NoozlerBomber,
            StarshipIcon::NoozlerFrigate => StarshipSprite::NoozlerFrigate,
            StarshipIcon::NoozlerBattleCruiser => StarshipSprite::NoozlerBattleCruiser,
            StarshipIcon::NoozlerDreadnought => StarshipSprite::NoozlerDreadnought,
            StarshipIcon::None => panic!("Must have an icon to convert"),
        }
    }
}
