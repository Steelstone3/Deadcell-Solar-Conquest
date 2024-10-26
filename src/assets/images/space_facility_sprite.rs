use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

use crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon;

use super::space_facility_type::SpaceFacilityType;

#[allow(clippy::enum_variant_names)]
#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SpaceFacilitySprite {
    AtarkSpaceStation,
    KarcanSpaceStation,
    NoozlerSpaceStation,
    GranokSpaceStation,
    AtarkSpaceShipConstructionYard,
    KarcanSpaceShipConstructionYard,
    NoozlerSpaceShipConstructionYard,
    GranokSpaceShipConstructionYard,
}

impl Display for SpaceFacilitySprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceFacilitySprite::AtarkSpaceStation => {
                write!(
                    formatter,
                    "images/factions/atark/space_facilities/atark_space_station.png"
                )
            }
            SpaceFacilitySprite::KarcanSpaceStation => {
                write!(
                    formatter,
                    "images/factions/karcan/space_facilities/karcan_space_station.png"
                )
            }
            SpaceFacilitySprite::NoozlerSpaceStation => {
                write!(
                    formatter,
                    "images/factions/noozler/space_facilities/noozler_space_station.png"
                )
            }
            SpaceFacilitySprite::GranokSpaceStation => {
                write!(
                    formatter,
                    "images/factions/granok/space_facilities/granok_space_station.png"
                )
            }
            SpaceFacilitySprite::AtarkSpaceShipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/atark/space_facilities/atark_spaceship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::KarcanSpaceShipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/karcan/space_facilities/karcan_spaceship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::NoozlerSpaceShipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/noozler/space_facilities/noozler_spaceship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::GranokSpaceShipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/granok/space_facilities/granok_spaceship_construction_yard.png"
                )
            }
        }
    }
}

impl SpaceFacilitySprite {
    pub fn sprite_convert_from(space_facility_icon: SpaceFacilityIcon) -> SpaceFacilitySprite {
        match space_facility_icon {
            SpaceFacilityIcon::AtarkSpaceShipConstructionYard => {
                SpaceFacilitySprite::AtarkSpaceShipConstructionYard
            }
            SpaceFacilityIcon::KarcanSpaceShipConstructionYard => {
                SpaceFacilitySprite::KarcanSpaceShipConstructionYard
            }
            SpaceFacilityIcon::NoozlerSpaceShipConstructionYard => {
                SpaceFacilitySprite::NoozlerSpaceShipConstructionYard
            }
            SpaceFacilityIcon::GranokSpaceShipConstructionYard => {
                SpaceFacilitySprite::GranokSpaceShipConstructionYard
            }
            SpaceFacilityIcon::None => {
                panic!("Space Facility Sprite: Must have an icon to convert")
            }
        }
    }

    pub fn space_facility_type_convert_from(
        space_facility_sprite: SpaceFacilitySprite,
    ) -> SpaceFacilityType {
        match space_facility_sprite {
            SpaceFacilitySprite::AtarkSpaceStation
            | SpaceFacilitySprite::KarcanSpaceStation
            | SpaceFacilitySprite::NoozlerSpaceStation
            | SpaceFacilitySprite::GranokSpaceStation => SpaceFacilityType::SpaceStation,
            SpaceFacilitySprite::AtarkSpaceShipConstructionYard
            | SpaceFacilitySprite::KarcanSpaceShipConstructionYard
            | SpaceFacilitySprite::NoozlerSpaceShipConstructionYard
            | SpaceFacilitySprite::GranokSpaceShipConstructionYard => {
                SpaceFacilityType::SpaceShipConstructionYard
            }
        }
    }
}

#[cfg(test)]
mod space_facility_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    #[should_panic(expected = "Space Facility Sprite: Must have an icon to convert")]
    fn sprite_convert_from_none() {
        // Given
        let space_facility_icon = SpaceFacilityIcon::None;

        // When
        SpaceFacilitySprite::sprite_convert_from(space_facility_icon);
    }

    #[rstest]
    #[case(
        SpaceFacilityIcon::AtarkSpaceShipConstructionYard,
        SpaceFacilitySprite::AtarkSpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilityIcon::KarcanSpaceShipConstructionYard,
        SpaceFacilitySprite::KarcanSpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilityIcon::NoozlerSpaceShipConstructionYard,
        SpaceFacilitySprite::NoozlerSpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilityIcon::GranokSpaceShipConstructionYard,
        SpaceFacilitySprite::GranokSpaceShipConstructionYard
    )]
    fn sprite_convert_from(
        #[case] space_facility_icon: SpaceFacilityIcon,
        #[case] space_facility_sprite: SpaceFacilitySprite,
    ) {
        // When
        let actual_space_facility_sprite =
            SpaceFacilitySprite::sprite_convert_from(space_facility_icon);

        // Then
        assert_eq!(space_facility_sprite, actual_space_facility_sprite);
    }

    #[rstest]
    #[case(
        SpaceFacilitySprite::AtarkSpaceShipConstructionYard,
        SpaceFacilityType::SpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilitySprite::KarcanSpaceShipConstructionYard,
        SpaceFacilityType::SpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilitySprite::NoozlerSpaceShipConstructionYard,
        SpaceFacilityType::SpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilitySprite::GranokSpaceShipConstructionYard,
        SpaceFacilityType::SpaceShipConstructionYard
    )]
    #[case(
        SpaceFacilitySprite::AtarkSpaceStation,
        SpaceFacilityType::SpaceStation
    )]
    #[case(
        SpaceFacilitySprite::KarcanSpaceStation,
        SpaceFacilityType::SpaceStation
    )]
    #[case(
        SpaceFacilitySprite::NoozlerSpaceStation,
        SpaceFacilityType::SpaceStation
    )]
    #[case(
        SpaceFacilitySprite::GranokSpaceStation,
        SpaceFacilityType::SpaceStation
    )]
    fn space_facility_type_convert_from(
        #[case] space_facility_sprite: SpaceFacilitySprite,
        #[case] space_facility_type: SpaceFacilityType,
    ) {
        // When
        let actual_space_facility_type =
            SpaceFacilitySprite::space_facility_type_convert_from(space_facility_sprite);

        // Then
        assert_eq!(space_facility_type, actual_space_facility_type)
    }
}
