use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SpaceFacilityIcon {
    AtarkStarshipConstructionYard,
    KarcanStarshipConstructionYard,
    NoozlerStarshipConstructionYard,
    GranokStarshipConstructionYard,
    None,
}

impl Display for SpaceFacilityIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceFacilityIcon::AtarkStarshipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/atark/atark_starship_construction_yard.png"
            ),
            SpaceFacilityIcon::KarcanStarshipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/karcan/karcan_starship_construction_yard.png"
            ),
            SpaceFacilityIcon::NoozlerStarshipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/noozler/noozler_starship_construction_yard.png"
            ),
            SpaceFacilityIcon::GranokStarshipConstructionYard => write!(
                formatter,
                "user_interface/icons/space_facilities/granok/granok_starship_construction_yard.png"
            ),
            SpaceFacilityIcon::None => write!(formatter, ""),
        }
    }
}
