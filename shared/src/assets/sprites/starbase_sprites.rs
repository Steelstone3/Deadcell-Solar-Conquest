use crate::resources::{factions::Factions, starbase_types::StarbaseTypes};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarbaseSprites {
    GranokImperialEmpireStarbase,
    StarGuardAllianceStarbase,
    UniversalMechanicalContigentDreadnoughtMothership,
    VoidwalkerCollectiveMothership,
    None,
}

impl Display for StarbaseSprites {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarbaseSprites::GranokImperialEmpireStarbase => write!(
                formatter,
                "images/factions/granok_imperial_empire/starbase.png"
            ),
            StarbaseSprites::StarGuardAllianceStarbase => write!(
                formatter,
                "images/factions/star_guard_alliance/space_station.png"
            ),
            StarbaseSprites::UniversalMechanicalContigentDreadnoughtMothership => write!(
                formatter,
                "images/factions/universal_mechanical_contingent/mothership.png"
            ),
            StarbaseSprites::VoidwalkerCollectiveMothership => write!(
                formatter,
                "images/factions/voidwalker_collective/mothership.png"
            ),
            StarbaseSprites::None => write!(formatter, ""),
        }
    }
}

#[allow(dead_code)]
pub fn sprite_convert_from(player_faction: Factions) -> StarbaseSprites {
    match player_faction {
        Factions::GranokImperialEmpire => StarbaseSprites::GranokImperialEmpireStarbase,
        Factions::StarGuardAlliance => StarbaseSprites::StarGuardAllianceStarbase,
        Factions::UniversalMechanicalContigent => {
            StarbaseSprites::UniversalMechanicalContigentDreadnoughtMothership
        }
        Factions::VoidwalkerCollective => StarbaseSprites::VoidwalkerCollectiveMothership,
        Factions::None => StarbaseSprites::None,
    }
}

#[allow(dead_code)]
pub fn starbase_type_convert_from(starbase_sprite: StarbaseSprites) -> StarbaseTypes {
    match starbase_sprite {
        StarbaseSprites::GranokImperialEmpireStarbase => StarbaseTypes::Starbase,
        StarbaseSprites::StarGuardAllianceStarbase => StarbaseTypes::Starbase,
        StarbaseSprites::UniversalMechanicalContigentDreadnoughtMothership => {
            StarbaseTypes::Mothership
        }
        StarbaseSprites::VoidwalkerCollectiveMothership => StarbaseTypes::Mothership,
        StarbaseSprites::None => StarbaseTypes::None,
    }
}
