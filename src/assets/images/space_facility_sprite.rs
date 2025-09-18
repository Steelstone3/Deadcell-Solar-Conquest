use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon;

use super::space_facility_type::SpaceFacilityType;

#[allow(clippy::enum_variant_names)]
#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy, Serialize, Deserialize)]
pub enum SpaceFacilitySprite {
    AtarkSpaceStation,
    KarcanSpaceStation,
    NoozlerSpaceStation,
    GranokSpaceStation,
    AtarkStarshipConstructionYard,
    KarcanStarshipConstructionYard,
    NoozlerStarshipConstructionYard,
    GranokStarshipConstructionYard,
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
            SpaceFacilitySprite::AtarkStarshipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/atark/space_facilities/atark_starship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::KarcanStarshipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/karcan/space_facilities/karcan_starship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::NoozlerStarshipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/noozler/space_facilities/noozler_starship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::GranokStarshipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/granok/space_facilities/granok_starship_construction_yard.png"
                )
            }
        }
    }
}

impl SpaceFacilitySprite {
    pub fn sprite_convert_from(space_facility_icon: SpaceFacilityIcon) -> SpaceFacilitySprite {
        match space_facility_icon {
            SpaceFacilityIcon::AtarkStarshipConstructionYard => {
                SpaceFacilitySprite::AtarkStarshipConstructionYard
            }
            SpaceFacilityIcon::KarcanStarshipConstructionYard => {
                SpaceFacilitySprite::KarcanStarshipConstructionYard
            }
            SpaceFacilityIcon::NoozlerStarshipConstructionYard => {
                SpaceFacilitySprite::NoozlerStarshipConstructionYard
            }
            SpaceFacilityIcon::GranokStarshipConstructionYard => {
                SpaceFacilitySprite::GranokStarshipConstructionYard
            }
            #[allow(clippy::panic)]
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
            SpaceFacilitySprite::AtarkStarshipConstructionYard
            | SpaceFacilitySprite::KarcanStarshipConstructionYard
            | SpaceFacilitySprite::NoozlerStarshipConstructionYard
            | SpaceFacilitySprite::GranokStarshipConstructionYard => {
                SpaceFacilityType::StarshipConstructionYard
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
        SpaceFacilityIcon::AtarkStarshipConstructionYard,
        SpaceFacilitySprite::AtarkStarshipConstructionYard
    )]
    #[case(
        SpaceFacilityIcon::KarcanStarshipConstructionYard,
        SpaceFacilitySprite::KarcanStarshipConstructionYard
    )]
    #[case(
        SpaceFacilityIcon::NoozlerStarshipConstructionYard,
        SpaceFacilitySprite::NoozlerStarshipConstructionYard
    )]
    #[case(
        SpaceFacilityIcon::GranokStarshipConstructionYard,
        SpaceFacilitySprite::GranokStarshipConstructionYard
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
        SpaceFacilitySprite::AtarkStarshipConstructionYard,
        SpaceFacilityType::StarshipConstructionYard
    )]
    #[case(
        SpaceFacilitySprite::KarcanStarshipConstructionYard,
        SpaceFacilityType::StarshipConstructionYard
    )]
    #[case(
        SpaceFacilitySprite::NoozlerStarshipConstructionYard,
        SpaceFacilityType::StarshipConstructionYard
    )]
    #[case(
        SpaceFacilitySprite::GranokStarshipConstructionYard,
        SpaceFacilityType::StarshipConstructionYard
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
