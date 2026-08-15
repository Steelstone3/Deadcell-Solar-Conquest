use crate::resources::faction::Faction;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarshipType {
    Corvette,
    Destroyer,
    Fighter,
    BattleCruiser,
    Battleship,
    TorpedoShip,
    IntelShip,
    Mothership,
    Dreadnought,
    None,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarshipSprite {
    GranokImperialEmpireCorvette,
    GranokImperialEmpireDestroyer,
    GranokImperialEmpireFighter,
    StarGuardAllianceBattleCruiser,
    StarGuardAllianceBattleship,
    StarGuardAllianceCorvette,
    StarGuardAllianceDestroyer,
    StarGuardAllianceTorpedoShip,
    UniversalMechanicalContigentDestroyer,
    UniversalMechanicalContigentIntelShip,
    VoidwalkerCollectiveDreadnought,
    VoidwalkerCollectiveFighter,
    None,
}

impl StarshipSprite {
    pub fn starship_type_convert_from(starship_sprite: StarshipSprite) -> StarshipType {
        match starship_sprite {
            StarshipSprite::GranokImperialEmpireCorvette => StarshipType::Corvette,
            StarshipSprite::GranokImperialEmpireDestroyer => StarshipType::Destroyer,
            StarshipSprite::GranokImperialEmpireFighter => StarshipType::Fighter,
            StarshipSprite::StarGuardAllianceBattleCruiser => StarshipType::BattleCruiser,
            StarshipSprite::StarGuardAllianceBattleship => StarshipType::Battleship,
            StarshipSprite::StarGuardAllianceCorvette => StarshipType::Corvette,
            StarshipSprite::StarGuardAllianceDestroyer => StarshipType::Destroyer,
            StarshipSprite::StarGuardAllianceTorpedoShip => StarshipType::TorpedoShip,
            StarshipSprite::UniversalMechanicalContigentDestroyer => StarshipType::Destroyer,
            StarshipSprite::UniversalMechanicalContigentIntelShip => StarshipType::IntelShip,
            StarshipSprite::VoidwalkerCollectiveDreadnought => StarshipType::Dreadnought,
            StarshipSprite::VoidwalkerCollectiveFighter => StarshipType::Fighter,
            StarshipSprite::None => StarshipType::None,
        }
    }

    pub fn sprite_convert_from(starship_type: StarshipType, faction: Faction) -> StarshipSprite {
        match faction {
            Faction::GranokImperialEmpire => match starship_type {
                StarshipType::Corvette => StarshipSprite::GranokImperialEmpireCorvette,
                StarshipType::Destroyer => StarshipSprite::GranokImperialEmpireDestroyer,
                StarshipType::Fighter => StarshipSprite::GranokImperialEmpireFighter,
                StarshipType::BattleCruiser => StarshipSprite::None,
                StarshipType::Battleship => StarshipSprite::None,
                StarshipType::TorpedoShip => StarshipSprite::None,
                StarshipType::IntelShip => StarshipSprite::None,
                StarshipType::Mothership => StarshipSprite::None,
                StarshipType::Dreadnought => StarshipSprite::None,
                StarshipType::None => StarshipSprite::None,
            },
            Faction::StarGuardAlliance => match starship_type {
                StarshipType::Corvette => StarshipSprite::GranokImperialEmpireCorvette,
                StarshipType::Destroyer => StarshipSprite::GranokImperialEmpireDestroyer,
                StarshipType::Fighter => StarshipSprite::GranokImperialEmpireFighter,
                StarshipType::BattleCruiser => StarshipSprite::None,
                StarshipType::Battleship => StarshipSprite::None,
                StarshipType::TorpedoShip => StarshipSprite::None,
                StarshipType::IntelShip => StarshipSprite::None,
                StarshipType::Mothership => StarshipSprite::None,
                StarshipType::Dreadnought => StarshipSprite::None,
                StarshipType::None => StarshipSprite::None,
            },
            Faction::UniversalMechanicalContigent => todo!(),
            Faction::VoidwalkerCollective => todo!(),
            Faction::None => todo!(),
        }
    }
}

impl Display for StarshipSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipSprite::GranokImperialEmpireCorvette => write!(
                formatter,
                "images/factions/granok_imperial_empire/corvette.png"
            ),
            StarshipSprite::GranokImperialEmpireDestroyer => write!(
                formatter,
                "images/factions/granok_imperial_empire/destroyer.png"
            ),
            StarshipSprite::GranokImperialEmpireFighter => write!(
                formatter,
                "images/factions/granok_imperial_empire/fighter.png"
            ),
            StarshipSprite::StarGuardAllianceBattleCruiser => write!(
                formatter,
                "images/factions/star_guard_alliance/battle_cruiser.png"
            ),
            StarshipSprite::StarGuardAllianceBattleship => write!(
                formatter,
                "images/factions/star_guard_alliance/battleship.png"
            ),
            StarshipSprite::StarGuardAllianceCorvette => write!(
                formatter,
                "images/factions/star_guard_alliance/corvette.png"
            ),
            StarshipSprite::StarGuardAllianceDestroyer => write!(
                formatter,
                "images/factions/star_guard_alliance/destroyer.png"
            ),
            StarshipSprite::StarGuardAllianceTorpedoShip => write!(
                formatter,
                "images/factions/star_guard_alliance/torpedo_ship.png"
            ),
            StarshipSprite::UniversalMechanicalContigentDestroyer => write!(
                formatter,
                "images/factions/universal_mechanical_contigent/destroyer.png"
            ),

            StarshipSprite::UniversalMechanicalContigentIntelShip => write!(
                formatter,
                "images/factions/universal_mechanical_contigent/intel_ship.png"
            ),
            StarshipSprite::VoidwalkerCollectiveDreadnought => write!(
                formatter,
                "images/factions/voidwalker_collective/dreadnought.png"
            ),
            StarshipSprite::VoidwalkerCollectiveFighter => write!(
                formatter,
                "images/factions/voidwalker_collective/fighter.png"
            ),
            StarshipSprite::None => write!(formatter, ""),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarbaseSprite {
    GranokImperialEmpireStarbase,
    StarGuardAllianceStarbase,
    UniversalMechanicalContigentDreadnoughtMothership,
    VoidwalkerCollectiveMothership,
    None,
}

// impl StarbaseSprite {
//     pub fn sprite_convert_from(
//         space_facility_icon: crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon,
//     ) -> StarbaseSprite {
//         match space_facility_icon {
//             crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon::AtarkStarshipConstructionYard => todo!(),
//             crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon::KarcanStarshipConstructionYard => todo!(),
//             crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon::NoozlerStarshipConstructionYard => todo!(),
//             crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon::GranokStarshipConstructionYard => todo!(),
//             crate::assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon::None => todo!(),
//         }
//     }
// }

impl Display for StarbaseSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarbaseSprite::GranokImperialEmpireStarbase => write!(
                formatter,
                "images/factions/granok_imperial_empire/starbase.png"
            ),
            StarbaseSprite::StarGuardAllianceStarbase => write!(
                formatter,
                "images/factions/star_guard_alliance/space_station.png"
            ),
            StarbaseSprite::UniversalMechanicalContigentDreadnoughtMothership => write!(
                formatter,
                "images/factions/universal_mechanical_contigent/dreadnought_mothership.png"
            ),
            StarbaseSprite::VoidwalkerCollectiveMothership => write!(
                formatter,
                "images/factions/voidwalker_collective/mothership.png"
            ),
            StarbaseSprite::None => write!(
                formatter,
                ""
            ),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarbaseType {
    Mothership,
    Starbase,
}

impl StarbaseType {
    pub fn sprite_convert_from(&self, player_faction: Faction) -> StarbaseSprite {
        match player_faction {
            Faction::GranokImperialEmpire => StarbaseSprite::GranokImperialEmpireStarbase,
            Faction::StarGuardAlliance => StarbaseSprite::StarGuardAllianceStarbase,
            Faction::UniversalMechanicalContigent => {
                StarbaseSprite::UniversalMechanicalContigentDreadnoughtMothership
            }
            Faction::VoidwalkerCollective => StarbaseSprite::VoidwalkerCollectiveMothership,
            Faction::None => StarbaseSprite::None,
        }
    }
}
