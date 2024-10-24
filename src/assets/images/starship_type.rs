use super::faction_starship_sprite::starship_sprite::StarshipSprite;
use crate::{
    assets::user_interface::icons::starship_icons::StarshipIcon, resources::faction::Faction,
};
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum StarshipType {
    SupportShip,
    Scout,
    Fighter,
    TorpedoShip,
    Bomber,
    Frigate,
    BattleCruiser,
    Dreadnought,
}

impl Display for StarshipType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipType::SupportShip => write!(formatter, "Support"),
            StarshipType::Scout => write!(formatter, "Scout"),
            StarshipType::Fighter => write!(formatter, "Fighter"),
            StarshipType::TorpedoShip => write!(formatter, "Torpedo"),
            StarshipType::Bomber => write!(formatter, "Bomber"),
            StarshipType::Frigate => write!(formatter, "Frigate"),
            StarshipType::BattleCruiser => write!(formatter, "BattleCruiser"),
            StarshipType::Dreadnought => write!(formatter, "Dreadnought"),
        }
    }
}

impl StarshipType {
    pub fn icon_convert_from(&self, faction: Faction) -> StarshipIcon {
        match faction {
            Faction::Atark => match self {
                StarshipType::SupportShip => StarshipIcon::AtarkSupportShip,
                StarshipType::Scout => StarshipIcon::AtarkScout,
                StarshipType::Fighter => StarshipIcon::AtarkFighter,
                StarshipType::TorpedoShip => StarshipIcon::AtarkTorpedoShip,
                StarshipType::Bomber => StarshipIcon::AtarkBomber,
                StarshipType::Frigate => StarshipIcon::AtarkFrigate,
                StarshipType::BattleCruiser => StarshipIcon::AtarkBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::AtarkDreadnought,
            },
            Faction::Karcan => match self {
                StarshipType::SupportShip => StarshipIcon::KarcanSupportShip,
                StarshipType::Scout => StarshipIcon::KarcanScout,
                StarshipType::Fighter => StarshipIcon::KarcanFighter,
                StarshipType::TorpedoShip => StarshipIcon::KarcanTorpedoShip,
                StarshipType::Bomber => StarshipIcon::KarcanBomber,
                StarshipType::Frigate => StarshipIcon::KarcanFrigate,
                StarshipType::BattleCruiser => StarshipIcon::KarcanBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::KarcanDreadnought,
            },
            Faction::Noozler => match self {
                StarshipType::SupportShip => StarshipIcon::NoozlerSupportShip,
                StarshipType::Scout => StarshipIcon::NoozlerScout,
                StarshipType::Fighter => StarshipIcon::NoozlerFighter,
                StarshipType::TorpedoShip => StarshipIcon::NoozlerTorpedoShip,
                StarshipType::Bomber => StarshipIcon::NoozlerBomber,
                StarshipType::Frigate => StarshipIcon::NoozlerFrigate,
                StarshipType::BattleCruiser => StarshipIcon::NoozlerBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::NoozlerDreadnought,
            },
            Faction::Granok => match self {
                StarshipType::SupportShip => StarshipIcon::GranokSupportShip,
                StarshipType::Scout => StarshipIcon::GranokScout,
                StarshipType::Fighter => StarshipIcon::GranokFighter,
                StarshipType::TorpedoShip => StarshipIcon::GranokTorpedoShip,
                StarshipType::Bomber => StarshipIcon::GranokBomber,
                StarshipType::Frigate => StarshipIcon::GranokFrigate,
                StarshipType::BattleCruiser => StarshipIcon::GranokBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::GranokDreadnought,
            },
        }
    }

    pub fn sprite_convert_from(&self, faction: Faction) -> StarshipSprite {
        match faction {
            Faction::Atark => match self {
                StarshipType::SupportShip => StarshipSprite::AtarkSupportShip,
                StarshipType::Scout => StarshipSprite::AtarkScout,
                StarshipType::Fighter => StarshipSprite::AtarkFighter,
                StarshipType::TorpedoShip => StarshipSprite::AtarkTorpedoShip,
                StarshipType::Bomber => StarshipSprite::AtarkBomber,
                StarshipType::Frigate => StarshipSprite::AtarkFrigate,
                StarshipType::BattleCruiser => StarshipSprite::AtarkBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::AtarkDreadnought,
            },
            Faction::Karcan => match self {
                StarshipType::SupportShip => StarshipSprite::KarcanSupportShip,
                StarshipType::Scout => StarshipSprite::KarcanScout,
                StarshipType::Fighter => StarshipSprite::KarcanFighter,
                StarshipType::TorpedoShip => StarshipSprite::KarcanTorpedoShip,
                StarshipType::Bomber => StarshipSprite::KarcanBomber,
                StarshipType::Frigate => StarshipSprite::KarcanFrigate,
                StarshipType::BattleCruiser => StarshipSprite::KarcanBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::KarcanDreadnought,
            },
            Faction::Noozler => match self {
                StarshipType::SupportShip => StarshipSprite::NoozlerSupportShip,
                StarshipType::Scout => StarshipSprite::NoozlerScout,
                StarshipType::Fighter => StarshipSprite::NoozlerFighter,
                StarshipType::TorpedoShip => StarshipSprite::NoozlerTorpedoShip,
                StarshipType::Bomber => StarshipSprite::NoozlerBomber,
                StarshipType::Frigate => StarshipSprite::NoozlerFrigate,
                StarshipType::BattleCruiser => StarshipSprite::NoozlerBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::NoozlerDreadnought,
            },
            Faction::Granok => match self {
                StarshipType::SupportShip => StarshipSprite::GranokSupportShip,
                StarshipType::Scout => StarshipSprite::GranokScout,
                StarshipType::Fighter => StarshipSprite::GranokFighter,
                StarshipType::TorpedoShip => StarshipSprite::GranokTorpedoShip,
                StarshipType::Bomber => StarshipSprite::GranokBomber,
                StarshipType::Frigate => StarshipSprite::GranokFrigate,
                StarshipType::BattleCruiser => StarshipSprite::GranokBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::GranokDreadnought,
            },
        }
    }
}

#[cfg(test)]
mod starship_type_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        StarshipType::SupportShip,
        Faction::Atark,
        StarshipSprite::AtarkSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Atark, StarshipSprite::AtarkScout)]
    #[case(StarshipType::Fighter, Faction::Atark, StarshipSprite::AtarkFighter)]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Atark,
        StarshipSprite::AtarkTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Atark, StarshipSprite::AtarkBomber)]
    #[case(StarshipType::Frigate, Faction::Atark, StarshipSprite::AtarkFrigate)]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Atark,
        StarshipSprite::AtarkBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Atark,
        StarshipSprite::AtarkDreadnought
    )]
    #[case(
        StarshipType::SupportShip,
        Faction::Karcan,
        StarshipSprite::KarcanSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Karcan, StarshipSprite::KarcanScout)]
    #[case(StarshipType::Fighter, Faction::Karcan, StarshipSprite::KarcanFighter)]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Karcan,
        StarshipSprite::KarcanTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Karcan, StarshipSprite::KarcanBomber)]
    #[case(StarshipType::Frigate, Faction::Karcan, StarshipSprite::KarcanFrigate)]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Karcan,
        StarshipSprite::KarcanBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Karcan,
        StarshipSprite::KarcanDreadnought
    )]
    #[case(
        StarshipType::SupportShip,
        Faction::Noozler,
        StarshipSprite::NoozlerSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Noozler, StarshipSprite::NoozlerScout)]
    #[case(
        StarshipType::Fighter,
        Faction::Noozler,
        StarshipSprite::NoozlerFighter
    )]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Noozler,
        StarshipSprite::NoozlerTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Noozler, StarshipSprite::NoozlerBomber)]
    #[case(
        StarshipType::Frigate,
        Faction::Noozler,
        StarshipSprite::NoozlerFrigate
    )]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Noozler,
        StarshipSprite::NoozlerBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Noozler,
        StarshipSprite::NoozlerDreadnought
    )]
    #[case(
        StarshipType::SupportShip,
        Faction::Granok,
        StarshipSprite::GranokSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Granok, StarshipSprite::GranokScout)]
    #[case(StarshipType::Fighter, Faction::Granok, StarshipSprite::GranokFighter)]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Granok,
        StarshipSprite::GranokTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Granok, StarshipSprite::GranokBomber)]
    #[case(StarshipType::Frigate, Faction::Granok, StarshipSprite::GranokFrigate)]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Granok,
        StarshipSprite::GranokBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Granok,
        StarshipSprite::GranokDreadnought
    )]
    fn sprite_convert_from(
        #[case] starship_type: StarshipType,
        #[case] faction: Faction,
        #[case] converted_type: StarshipSprite,
    ) {
        // When
        let actual_converted_type = starship_type.sprite_convert_from(faction);

        // Then
        assert_eq!(converted_type, actual_converted_type);
    }

    #[rstest]
    #[case(
        StarshipType::SupportShip,
        Faction::Atark,
        StarshipIcon::AtarkSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Atark, StarshipIcon::AtarkScout)]
    #[case(StarshipType::Fighter, Faction::Atark, StarshipIcon::AtarkFighter)]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Atark,
        StarshipIcon::AtarkTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Atark, StarshipIcon::AtarkBomber)]
    #[case(StarshipType::Frigate, Faction::Atark, StarshipIcon::AtarkFrigate)]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Atark,
        StarshipIcon::AtarkBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Atark,
        StarshipIcon::AtarkDreadnought
    )]
    #[case(
        StarshipType::SupportShip,
        Faction::Karcan,
        StarshipIcon::KarcanSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Karcan, StarshipIcon::KarcanScout)]
    #[case(StarshipType::Fighter, Faction::Karcan, StarshipIcon::KarcanFighter)]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Karcan,
        StarshipIcon::KarcanTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Karcan, StarshipIcon::KarcanBomber)]
    #[case(StarshipType::Frigate, Faction::Karcan, StarshipIcon::KarcanFrigate)]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Karcan,
        StarshipIcon::KarcanBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Karcan,
        StarshipIcon::KarcanDreadnought
    )]
    #[case(
        StarshipType::SupportShip,
        Faction::Noozler,
        StarshipIcon::NoozlerSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Noozler, StarshipIcon::NoozlerScout)]
    #[case(StarshipType::Fighter, Faction::Noozler, StarshipIcon::NoozlerFighter)]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Noozler,
        StarshipIcon::NoozlerTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Noozler, StarshipIcon::NoozlerBomber)]
    #[case(StarshipType::Frigate, Faction::Noozler, StarshipIcon::NoozlerFrigate)]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Noozler,
        StarshipIcon::NoozlerBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Noozler,
        StarshipIcon::NoozlerDreadnought
    )]
    #[case(
        StarshipType::SupportShip,
        Faction::Granok,
        StarshipIcon::GranokSupportShip
    )]
    #[case(StarshipType::Scout, Faction::Granok, StarshipIcon::GranokScout)]
    #[case(StarshipType::Fighter, Faction::Granok, StarshipIcon::GranokFighter)]
    #[case(
        StarshipType::TorpedoShip,
        Faction::Granok,
        StarshipIcon::GranokTorpedoShip
    )]
    #[case(StarshipType::Bomber, Faction::Granok, StarshipIcon::GranokBomber)]
    #[case(StarshipType::Frigate, Faction::Granok, StarshipIcon::GranokFrigate)]
    #[case(
        StarshipType::BattleCruiser,
        Faction::Granok,
        StarshipIcon::GranokBattleCruiser
    )]
    #[case(
        StarshipType::Dreadnought,
        Faction::Granok,
        StarshipIcon::GranokDreadnought
    )]
    fn icon_convert_from(
        #[case] starship_type: StarshipType,
        #[case] faction: Faction,
        #[case] converted_type: StarshipIcon,
    ) {
        // When
        let actual_converted_type = starship_type.icon_convert_from(faction);

        // Then
        assert_eq!(converted_type, actual_converted_type);
    }
}
