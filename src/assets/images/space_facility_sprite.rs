use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

use crate::assets::user_interace::icons::space_facility_icons::SpaceFacilityIcon;

#[allow(clippy::enum_variant_names)]
#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SpaceStationSprite {
    AtarkSpaceStation,
    KarcanSpaceStation,
    NoozlerSpaceStation,
}

impl Display for SpaceStationSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceStationSprite::AtarkSpaceStation => {
                write!(
                    formatter,
                    "images/factions/atark/space_facilities/atark_space_station.png"
                )
            }
            SpaceStationSprite::KarcanSpaceStation => {
                write!(
                    formatter,
                    "images/factions/karcan/space_facilities/karcan_space_station.png"
                )
            }
            SpaceStationSprite::NoozlerSpaceStation => {
                write!(
                    formatter,
                    "images/factions/noozler/space_facilities/noozler_space_station.png"
                )
            }
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SpaceFacilitySprite {
    AtarkSpaceShipConstructionYard,
    KarcanSpaceShipConstructionYard,
    NoozlerSpaceShipConstructionYard,
}

impl Display for SpaceFacilitySprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            SpaceFacilityIcon::None => {
                panic!("Space Facility Sprite: Must have an icon to convert")
            }
        }
    }
}
