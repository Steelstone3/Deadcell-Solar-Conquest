use crate::assets::{
    images::starship_type::StarshipType, user_interface::icons::starship_icons::StarshipIcon,
};
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum StarshipSprite {
    AtarkSupportShip,
    AtarkScout,
    AtarkFighter,
    AtarkTorpedoShip,
    AtarkBomber,
    AtarkFrigate,
    AtarkBattleCruiser,
    AtarkDreadnought,
    KarcanSupportShip,
    KarcanScout,
    KarcanFighter,
    KarcanTorpedoShip,
    KarcanBomber,
    KarcanFrigate,
    KarcanBattleCruiser,
    KarcanDreadnought,
    NoozlerSupportShip,
    NoozlerScout,
    NoozlerFighter,
    NoozlerTorpedoShip,
    NoozlerBomber,
    NoozlerFrigate,
    NoozlerBattleCruiser,
    NoozlerDreadnought,
    GranokSupportShip,
    GranokScout,
    GranokFighter,
    GranokTorpedoShip,
    GranokBomber,
    GranokFrigate,
    GranokBattleCruiser,
    GranokDreadnought,
}

impl Display for StarshipSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/support_ship/atark_support_ship.png"
            ),
            StarshipSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/scout/atark_scout.png"
            ),
            StarshipSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/fighter/atark_fighter.png"
            ),
            StarshipSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/torpedo_ship/atark_torpedo_ship.png"
            ),
            StarshipSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/bomber/atark_bomber.png"
            ),
            StarshipSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/frigate/atark_frigate.png"
            ),
            StarshipSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser.png"
            ),
            StarshipSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/dreadnought/atark_dreadnought.png"
            ),
            StarshipSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/support_ship/karcan_support_ship.png"
            ),
            StarshipSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/scout/karcan_scout.png"
            ),
            StarshipSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/fighter/karcan_fighter.png"
            ),
            StarshipSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/torpedo_ship/karcan_torpedo_ship.png"
            ),
            StarshipSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/bomber/karcan_bomber.png"
            ),
            StarshipSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/frigate/karcan_frigate.png"
            ),
            StarshipSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser.png"
            ),
            StarshipSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/dreadnought/karcan_dreadnought.png"
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
            StarshipSprite::GranokBattleCruiser => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokBomber => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokDreadnought => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokFighter => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokFrigate => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokScout => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokSupportShip => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokTorpedoShip => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
        }
    }
}

impl StarshipSprite {
    pub fn sprite_convert_from(starship_icon: StarshipIcon) -> StarshipSprite {
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
            StarshipIcon::GranokSupportShip => StarshipSprite::GranokSupportShip,
            StarshipIcon::GranokScout => StarshipSprite::GranokScout,
            StarshipIcon::GranokFighter => StarshipSprite::GranokFighter,
            StarshipIcon::GranokTorpedoShip => StarshipSprite::GranokTorpedoShip,
            StarshipIcon::GranokBomber => StarshipSprite::GranokBomber,
            StarshipIcon::GranokFrigate => StarshipSprite::GranokFrigate,
            StarshipIcon::GranokBattleCruiser => StarshipSprite::GranokBattleCruiser,
            StarshipIcon::GranokDreadnought => StarshipSprite::GranokDreadnought,
            StarshipIcon::None => panic!("Spaceship Sprite: Must have an icon to convert"),
        }
    }

    pub fn starship_type_convert_from(starship_sprite: StarshipSprite) -> StarshipType {
        match starship_sprite {
            StarshipSprite::AtarkSupportShip
            | StarshipSprite::KarcanSupportShip
            | StarshipSprite::NoozlerSupportShip
            | StarshipSprite::GranokSupportShip => StarshipType::SupportShip,
            StarshipSprite::AtarkScout
            | StarshipSprite::KarcanScout
            | StarshipSprite::NoozlerScout
            | StarshipSprite::GranokScout => StarshipType::Scout,
            StarshipSprite::AtarkFighter
            | StarshipSprite::KarcanFighter
            | StarshipSprite::NoozlerFighter
            | StarshipSprite::GranokFighter => StarshipType::Fighter,
            StarshipSprite::AtarkTorpedoShip
            | StarshipSprite::KarcanTorpedoShip
            | StarshipSprite::NoozlerTorpedoShip
            | StarshipSprite::GranokTorpedoShip => StarshipType::TorpedoShip,
            StarshipSprite::AtarkBomber
            | StarshipSprite::KarcanBomber
            | StarshipSprite::NoozlerBomber
            | StarshipSprite::GranokBomber => StarshipType::Bomber,
            StarshipSprite::AtarkFrigate
            | StarshipSprite::KarcanFrigate
            | StarshipSprite::NoozlerFrigate
            | StarshipSprite::GranokFrigate => StarshipType::Frigate,
            StarshipSprite::AtarkBattleCruiser
            | StarshipSprite::KarcanBattleCruiser
            | StarshipSprite::NoozlerBattleCruiser
            | StarshipSprite::GranokBattleCruiser => StarshipType::BattleCruiser,
            StarshipSprite::AtarkDreadnought
            | StarshipSprite::KarcanDreadnought
            | StarshipSprite::NoozlerDreadnought
            | StarshipSprite::GranokDreadnought => StarshipType::Dreadnought,
        }
    }
}

#[cfg(test)]
mod starship_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    #[should_panic(expected = "Spaceship Sprite: Must have an icon to convert")]
    fn sprite_convert_from_none() {
        // Given
        let starship_icon = StarshipIcon::None;

        // When
        StarshipSprite::sprite_convert_from(starship_icon);
    }

    #[rstest]
    #[case(StarshipIcon::AtarkSupportShip, StarshipSprite::AtarkSupportShip)]
    #[case(StarshipIcon::AtarkScout, StarshipSprite::AtarkScout)]
    #[case(StarshipIcon::AtarkFighter, StarshipSprite::AtarkFighter)]
    #[case(StarshipIcon::AtarkTorpedoShip, StarshipSprite::AtarkTorpedoShip)]
    #[case(StarshipIcon::AtarkBomber, StarshipSprite::AtarkBomber)]
    #[case(StarshipIcon::AtarkFrigate, StarshipSprite::AtarkFrigate)]
    #[case(StarshipIcon::AtarkBattleCruiser, StarshipSprite::AtarkBattleCruiser)]
    #[case(StarshipIcon::AtarkDreadnought, StarshipSprite::AtarkDreadnought)]
    #[case(StarshipIcon::KarcanSupportShip, StarshipSprite::KarcanSupportShip)]
    #[case(StarshipIcon::KarcanScout, StarshipSprite::KarcanScout)]
    #[case(StarshipIcon::KarcanFighter, StarshipSprite::KarcanFighter)]
    #[case(StarshipIcon::KarcanTorpedoShip, StarshipSprite::KarcanTorpedoShip)]
    #[case(StarshipIcon::KarcanBomber, StarshipSprite::KarcanBomber)]
    #[case(StarshipIcon::KarcanFrigate, StarshipSprite::KarcanFrigate)]
    #[case(StarshipIcon::KarcanBattleCruiser, StarshipSprite::KarcanBattleCruiser)]
    #[case(StarshipIcon::KarcanDreadnought, StarshipSprite::KarcanDreadnought)]
    #[case(StarshipIcon::NoozlerSupportShip, StarshipSprite::NoozlerSupportShip)]
    #[case(StarshipIcon::NoozlerScout, StarshipSprite::NoozlerScout)]
    #[case(StarshipIcon::NoozlerFighter, StarshipSprite::NoozlerFighter)]
    #[case(StarshipIcon::NoozlerTorpedoShip, StarshipSprite::NoozlerTorpedoShip)]
    #[case(StarshipIcon::NoozlerBomber, StarshipSprite::NoozlerBomber)]
    #[case(StarshipIcon::NoozlerFrigate, StarshipSprite::NoozlerFrigate)]
    #[case(
        StarshipIcon::NoozlerBattleCruiser,
        StarshipSprite::NoozlerBattleCruiser
    )]
    #[case(StarshipIcon::NoozlerDreadnought, StarshipSprite::NoozlerDreadnought)]
    #[case(StarshipIcon::GranokSupportShip, StarshipSprite::GranokSupportShip)]
    #[case(StarshipIcon::GranokScout, StarshipSprite::GranokScout)]
    #[case(StarshipIcon::GranokFighter, StarshipSprite::GranokFighter)]
    #[case(StarshipIcon::GranokTorpedoShip, StarshipSprite::GranokTorpedoShip)]
    #[case(StarshipIcon::GranokBomber, StarshipSprite::GranokBomber)]
    #[case(StarshipIcon::GranokFrigate, StarshipSprite::GranokFrigate)]
    #[case(StarshipIcon::GranokBattleCruiser, StarshipSprite::GranokBattleCruiser)]
    #[case(StarshipIcon::GranokDreadnought, StarshipSprite::GranokDreadnought)]
    fn sprite_convert_from(
        #[case] starship_icon: StarshipIcon,
        #[case] starship_sprite: StarshipSprite,
    ) {
        // When
        let actual_starship_sprite = StarshipSprite::sprite_convert_from(starship_icon);

        // Then
        assert_eq!(starship_sprite, actual_starship_sprite);
    }

    #[rstest]
    #[case(StarshipSprite::AtarkSupportShip, StarshipType::SupportShip)]
    #[case(StarshipSprite::AtarkScout, StarshipType::Scout)]
    #[case(StarshipSprite::AtarkFighter, StarshipType::Fighter)]
    #[case(StarshipSprite::AtarkTorpedoShip, StarshipType::TorpedoShip)]
    #[case(StarshipSprite::AtarkBomber, StarshipType::Bomber)]
    #[case(StarshipSprite::AtarkFrigate, StarshipType::Frigate)]
    #[case(StarshipSprite::AtarkBattleCruiser, StarshipType::BattleCruiser)]
    #[case(StarshipSprite::AtarkDreadnought, StarshipType::Dreadnought)]
    #[case(StarshipSprite::KarcanSupportShip, StarshipType::SupportShip)]
    #[case(StarshipSprite::KarcanScout, StarshipType::Scout)]
    #[case(StarshipSprite::KarcanFighter, StarshipType::Fighter)]
    #[case(StarshipSprite::KarcanTorpedoShip, StarshipType::TorpedoShip)]
    #[case(StarshipSprite::KarcanBomber, StarshipType::Bomber)]
    #[case(StarshipSprite::KarcanFrigate, StarshipType::Frigate)]
    #[case(StarshipSprite::KarcanBattleCruiser, StarshipType::BattleCruiser)]
    #[case(StarshipSprite::KarcanDreadnought, StarshipType::Dreadnought)]
    #[case(StarshipSprite::NoozlerSupportShip, StarshipType::SupportShip)]
    #[case(StarshipSprite::NoozlerScout, StarshipType::Scout)]
    #[case(StarshipSprite::NoozlerFighter, StarshipType::Fighter)]
    #[case(StarshipSprite::NoozlerTorpedoShip, StarshipType::TorpedoShip)]
    #[case(StarshipSprite::NoozlerBomber, StarshipType::Bomber)]
    #[case(StarshipSprite::NoozlerFrigate, StarshipType::Frigate)]
    #[case(StarshipSprite::NoozlerBattleCruiser, StarshipType::BattleCruiser)]
    #[case(StarshipSprite::NoozlerDreadnought, StarshipType::Dreadnought)]
    #[case(StarshipSprite::GranokSupportShip, StarshipType::SupportShip)]
    #[case(StarshipSprite::GranokScout, StarshipType::Scout)]
    #[case(StarshipSprite::GranokFighter, StarshipType::Fighter)]
    #[case(StarshipSprite::GranokTorpedoShip, StarshipType::TorpedoShip)]
    #[case(StarshipSprite::GranokBomber, StarshipType::Bomber)]
    #[case(StarshipSprite::GranokFrigate, StarshipType::Frigate)]
    #[case(StarshipSprite::GranokBattleCruiser, StarshipType::BattleCruiser)]
    #[case(StarshipSprite::GranokDreadnought, StarshipType::Dreadnought)]
    fn starship_type_convert_from(
        #[case] starship_sprite: StarshipSprite,
        #[case] starship_type: StarshipType,
    ) {
        // When
        let actual_starship_type = StarshipSprite::starship_type_convert_from(starship_sprite);

        // Then
        assert_eq!(starship_type, actual_starship_type)
    }
}
