use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SpaceFacilityIcon {
    AtarkSpaceShipConstructionYard,
    KarcanSpaceShipConstructionYard,
    NoozlerSpaceShipConstructionYard,
    GranokSpaceShipConstructionYard,
    None,
}

impl Display for SpaceFacilityIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceFacilityIcon::AtarkSpaceShipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/atark/atark_spaceship_construction_yard.png"
            ),
            SpaceFacilityIcon::KarcanSpaceShipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/karcan/karcan_spaceship_construction_yard.png"
            ),
            SpaceFacilityIcon::NoozlerSpaceShipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/noozler/noozler_spaceship_construction_yard.png"
            ),
            SpaceFacilityIcon::GranokSpaceShipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/granok/granok_spaceship_construction_yard.png"
            ),
            SpaceFacilityIcon::None => write!(formatter, ""),
        }
    }
}
