use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

use crate::resources::faction::Faction;

// TODO consider alternatative assets
#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum CommanderIcon {
    GranokImperialEmpireCommander,
    StarGuardAllianceCommander,
    UniversalMechanicalContigentCommander,
    VoidwalkerCollectiveCommander,
    None,
}

impl Display for CommanderIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommanderIcon::None => {
                write!(formatter, "")
            }
            CommanderIcon::GranokImperialEmpireCommander => write!(
                formatter,
                "images/factions/granok_imperial_empire/granok_imperial_empire_commander.jpg"
            ),
            CommanderIcon::StarGuardAllianceCommander => write!(
                formatter,
                "images/factions/star_guard_alliance/star_guard_alliance_commander.jpg"
            ),
            CommanderIcon::UniversalMechanicalContigentCommander => write!(
                formatter,
                "images/factions/universal_mechanical_contingent/universal_mechanical_contingent_commander.jpg"
            ),
            CommanderIcon::VoidwalkerCollectiveCommander => write!(
                formatter,
                "images/factions/voidwalker_collective/voidwalker_collective_commander.jpg"
            ),
        }
    }
}

impl CommanderIcon {
    pub fn convert_from(faction: Faction) -> CommanderIcon {
        match faction {
            Faction::GranokImperialEmpire => todo!(),
            Faction::StarGuardAlliance => todo!(),
            Faction::UniversalMechanicalContigent => todo!(),
            Faction::VoidwalkerCollective => todo!(),
            Faction::None => todo!(),
        }
    }
}

// #[cfg(test)]
// mod commander_icons_should {
//     use super::*;
//     use rstest::rstest;

//     #[rstest]
//     #[case(Faction::Atark, CommanderIcon::AtarkCommander)]
//     #[case(Faction::Karcan, CommanderIcon::KaranCommander)]
//     #[case(Faction::Noozler, CommanderIcon::NoozlerCommander)]
//     #[case(Faction::Granok, CommanderIcon::GranokCommander)]
//     fn convert_from(#[case] faction: Faction, #[case] commander_icon: CommanderIcon) {
//         // When
//         let actual_commander_icon = CommanderIcon::convert_from(faction);

//         // Then
//         assert_eq!(commander_icon, actual_commander_icon);
//     }
// }
