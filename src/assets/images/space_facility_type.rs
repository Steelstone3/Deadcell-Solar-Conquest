use super::space_facility_sprite::SpaceFacilitySprite;
use crate::{
    assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon,
    resources::faction::Faction,
};

#[derive(PartialEq, Debug)]
pub enum SpaceFacilityType {
    SpaceStation,
    SpaceShipConstructionYard,
}

impl SpaceFacilityType {
    pub fn icon_convert_from(&self, faction: Faction) -> SpaceFacilityIcon {
        match faction {
            Faction::Atark => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::AtarkSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => panic!("No space station icon"),
            },
            Faction::Karcan => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::KarcanSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => panic!("No space station icon"),
            },
            Faction::Noozler => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::NoozlerSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => panic!("No space station icon"),
            },
            Faction::Granok => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::GranokSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => panic!("No space station icon"),
            },
        }
    }

    pub fn sprite_convert_from(&self, faction: Faction) -> SpaceFacilitySprite {
        match faction {
            Faction::Atark => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilitySprite::AtarkSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => SpaceFacilitySprite::AtarkSpaceStation,
            },
            Faction::Karcan => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilitySprite::KarcanSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => SpaceFacilitySprite::KarcanSpaceStation,
            },
            Faction::Noozler => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilitySprite::NoozlerSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => SpaceFacilitySprite::NoozlerSpaceStation,
            },
            Faction::Granok => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilitySprite::GranokSpaceShipConstructionYard
                }
                SpaceFacilityType::SpaceStation => SpaceFacilitySprite::GranokSpaceStation,
            },
        }
    }
}

#[cfg(test)]
mod space_facility_type_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[should_panic(expected = "No space station icon")]
    #[case::panic(SpaceFacilityType::SpaceStation, Faction::Atark)]
    #[should_panic(expected = "No space station icon")]
    #[case::panic(SpaceFacilityType::SpaceStation, Faction::Karcan)]
    #[should_panic(expected = "No space station icon")]
    #[case::panic(SpaceFacilityType::SpaceStation, Faction::Noozler)]
    #[should_panic(expected = "No space station icon")]
    #[case::panic(SpaceFacilityType::SpaceStation, Faction::Granok)]
    fn icon_convert_from_space_station(
        #[case] space_facility_type: SpaceFacilityType,
        #[case] faction: Faction,
    ) {
        // When
        space_facility_type.icon_convert_from(faction);
    }

    #[rstest]
    #[case::panic(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Atark,
        SpaceFacilityIcon::AtarkSpaceShipConstructionYard
    )]
    #[case::panic(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Karcan,
        SpaceFacilityIcon::KarcanSpaceShipConstructionYard
    )]
    #[case::panic(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Noozler,
        SpaceFacilityIcon::NoozlerSpaceShipConstructionYard
    )]
    #[case::panic(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Granok,
        SpaceFacilityIcon::GranokSpaceShipConstructionYard
    )]
    fn icon_convert_from(
        #[case] space_facility_type: SpaceFacilityType,
        #[case] faction: Faction,
        #[case] space_facility_icon: SpaceFacilityIcon,
    ) {
        // When
        let actual_space_facility_icon = space_facility_type.icon_convert_from(faction);

        // Then
        assert_eq!(space_facility_icon, actual_space_facility_icon);
    }

    #[rstest]
    #[case(
        SpaceFacilityType::SpaceStation,
        Faction::Atark,
        SpaceFacilitySprite::AtarkSpaceStation
    )]
    #[case(
        SpaceFacilityType::SpaceStation,
        Faction::Karcan,
        SpaceFacilitySprite::KarcanSpaceStation
    )]
    #[case(
        SpaceFacilityType::SpaceStation,
        Faction::Noozler,
        SpaceFacilitySprite::NoozlerSpaceStation
    )]
    #[case(
        SpaceFacilityType::SpaceStation,
        Faction::Granok,
        SpaceFacilitySprite::GranokSpaceStation
    )]
    #[case(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Atark,
        SpaceFacilitySprite::AtarkSpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Karcan,
        SpaceFacilitySprite::KarcanSpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Noozler,
        SpaceFacilitySprite::NoozlerSpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilityType::SpaceShipConstructionYard,
        Faction::Granok,
        SpaceFacilitySprite::GranokSpaceShipConstructionYard
    )]
    fn sprite_convert_from(
        #[case] space_facility_type: SpaceFacilityType,
        #[case] faction: Faction,
        #[case] space_facility_sprite: SpaceFacilitySprite,
    ) {
        // When
        let actual_space_facility_sprite = space_facility_type.sprite_convert_from(faction);

        // Then
        assert_eq!(space_facility_sprite, actual_space_facility_sprite);
    }
}
