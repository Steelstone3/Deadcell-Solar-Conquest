use deadcell_solar_conquest_shared::resources::{
    factions::Factions, starship_types::StarshipTypes,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarshipSprites {
    GranokImperialEmpireCorvette,
    GranokImperialEmpireDestroyer,
    GranokImperialEmpireFighter,
    StarGuardAllianceBattleCruiser,
    StarGuardAllianceBattleship,
    StarGuardAllianceCorvette,
    StarGuardAllianceDestroyer,
    StarGuardAllianceTorpedoShip,
    UniversalMechanicalContingentDestroyer,
    UniversalMechanicalContingentIntelShip,
    VoidwalkerCollectiveDreadnought,
    VoidwalkerCollectiveFighter,
    None,
}

pub fn starship_type_convert_from(starship_sprite: StarshipSprites) -> StarshipTypes {
    match starship_sprite {
        StarshipSprites::GranokImperialEmpireCorvette => StarshipTypes::Corvette,
        StarshipSprites::GranokImperialEmpireDestroyer => StarshipTypes::Destroyer,
        StarshipSprites::GranokImperialEmpireFighter => StarshipTypes::Fighter,
        StarshipSprites::StarGuardAllianceBattleCruiser => StarshipTypes::BattleCruiser,
        StarshipSprites::StarGuardAllianceBattleship => StarshipTypes::Battleship,
        StarshipSprites::StarGuardAllianceCorvette => StarshipTypes::Corvette,
        StarshipSprites::StarGuardAllianceDestroyer => StarshipTypes::Destroyer,
        StarshipSprites::StarGuardAllianceTorpedoShip => StarshipTypes::TorpedoShip,
        StarshipSprites::UniversalMechanicalContingentDestroyer => StarshipTypes::Destroyer,
        StarshipSprites::UniversalMechanicalContingentIntelShip => StarshipTypes::IntelShip,
        StarshipSprites::VoidwalkerCollectiveDreadnought => StarshipTypes::Dreadnought,
        StarshipSprites::VoidwalkerCollectiveFighter => StarshipTypes::Fighter,
        StarshipSprites::None => StarshipTypes::None,
    }
}

pub fn sprite_convert_from(starship_type: StarshipTypes, faction: Factions) -> StarshipSprites {
    match faction {
        Factions::GranokImperialEmpire => match starship_type {
            StarshipTypes::Corvette => StarshipSprites::GranokImperialEmpireCorvette,
            StarshipTypes::Destroyer => StarshipSprites::GranokImperialEmpireDestroyer,
            StarshipTypes::Fighter => StarshipSprites::GranokImperialEmpireFighter,
            StarshipTypes::BattleCruiser => StarshipSprites::None,
            StarshipTypes::Battleship => StarshipSprites::None,
            StarshipTypes::TorpedoShip => StarshipSprites::None,
            StarshipTypes::IntelShip => StarshipSprites::None,
            StarshipTypes::Mothership => StarshipSprites::None,
            StarshipTypes::Dreadnought => StarshipSprites::None,
            StarshipTypes::None => StarshipSprites::None,
        },
        Factions::StarGuardAlliance => match starship_type {
            StarshipTypes::Corvette => StarshipSprites::StarGuardAllianceCorvette,
            StarshipTypes::Destroyer => StarshipSprites::StarGuardAllianceDestroyer,
            StarshipTypes::Fighter => StarshipSprites::None,
            StarshipTypes::BattleCruiser => StarshipSprites::StarGuardAllianceBattleCruiser,
            StarshipTypes::Battleship => StarshipSprites::StarGuardAllianceBattleship,
            StarshipTypes::TorpedoShip => StarshipSprites::StarGuardAllianceTorpedoShip,
            StarshipTypes::IntelShip => StarshipSprites::None,
            StarshipTypes::Mothership => StarshipSprites::None,
            StarshipTypes::Dreadnought => StarshipSprites::None,
            StarshipTypes::None => StarshipSprites::None,
        },
        Factions::UniversalMechanicalContigent => match starship_type {
            StarshipTypes::Corvette => StarshipSprites::None,
            StarshipTypes::Destroyer => StarshipSprites::UniversalMechanicalContingentDestroyer,
            StarshipTypes::Fighter => StarshipSprites::None,
            StarshipTypes::BattleCruiser => StarshipSprites::None,
            StarshipTypes::Battleship => StarshipSprites::None,
            StarshipTypes::TorpedoShip => StarshipSprites::None,
            StarshipTypes::IntelShip => StarshipSprites::UniversalMechanicalContingentIntelShip,
            StarshipTypes::Mothership => StarshipSprites::None,
            StarshipTypes::Dreadnought => StarshipSprites::None,
            StarshipTypes::None => StarshipSprites::None,
        },
        Factions::VoidwalkerCollective => match starship_type {
            StarshipTypes::Corvette => StarshipSprites::None,
            StarshipTypes::Destroyer => StarshipSprites::None,
            StarshipTypes::Fighter => StarshipSprites::VoidwalkerCollectiveFighter,
            StarshipTypes::BattleCruiser => StarshipSprites::None,
            StarshipTypes::Battleship => StarshipSprites::None,
            StarshipTypes::TorpedoShip => StarshipSprites::None,
            StarshipTypes::IntelShip => StarshipSprites::None,
            StarshipTypes::Mothership => StarshipSprites::None,
            StarshipTypes::Dreadnought => StarshipSprites::VoidwalkerCollectiveDreadnought,
            StarshipTypes::None => StarshipSprites::None,
        },
        Factions::None => match starship_type {
            StarshipTypes::Corvette => StarshipSprites::None,
            StarshipTypes::Destroyer => StarshipSprites::None,
            StarshipTypes::Fighter => StarshipSprites::None,
            StarshipTypes::BattleCruiser => StarshipSprites::None,
            StarshipTypes::Battleship => StarshipSprites::None,
            StarshipTypes::TorpedoShip => StarshipSprites::None,
            StarshipTypes::IntelShip => StarshipSprites::None,
            StarshipTypes::Mothership => StarshipSprites::None,
            StarshipTypes::Dreadnought => StarshipSprites::None,
            StarshipTypes::None => StarshipSprites::None,
        },
    }
}

impl Display for StarshipSprites {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipSprites::GranokImperialEmpireCorvette => write!(
                formatter,
                "images/factions/granok_imperial_empire/corvette.png"
            ),
            StarshipSprites::GranokImperialEmpireDestroyer => write!(
                formatter,
                "images/factions/granok_imperial_empire/destroyer.png"
            ),
            StarshipSprites::GranokImperialEmpireFighter => write!(
                formatter,
                "images/factions/granok_imperial_empire/fighter.png"
            ),
            StarshipSprites::StarGuardAllianceBattleCruiser => write!(
                formatter,
                "images/factions/star_guard_alliance/battle_cruiser.png"
            ),
            StarshipSprites::StarGuardAllianceBattleship => write!(
                formatter,
                "images/factions/star_guard_alliance/battleship.png"
            ),
            StarshipSprites::StarGuardAllianceCorvette => write!(
                formatter,
                "images/factions/star_guard_alliance/corvette.png"
            ),
            StarshipSprites::StarGuardAllianceDestroyer => write!(
                formatter,
                "images/factions/star_guard_alliance/destroyer.png"
            ),
            StarshipSprites::StarGuardAllianceTorpedoShip => write!(
                formatter,
                "images/factions/star_guard_alliance/torpedo_ship.png"
            ),
            StarshipSprites::UniversalMechanicalContingentDestroyer => write!(
                formatter,
                "images/factions/universal_mechanical_contingent/destroyer.png"
            ),
            StarshipSprites::UniversalMechanicalContingentIntelShip => write!(
                formatter,
                "images/factions/universal_mechanical_contingent/intel_ship.png"
            ),
            StarshipSprites::VoidwalkerCollectiveDreadnought => write!(
                formatter,
                "images/factions/voidwalker_collective/dreadnought.png"
            ),
            StarshipSprites::VoidwalkerCollectiveFighter => write!(
                formatter,
                "images/factions/voidwalker_collective/fighter.png"
            ),
            StarshipSprites::None => write!(formatter, ""),
        }
    }
}
