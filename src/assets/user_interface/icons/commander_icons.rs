use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

use crate::resources::faction::Faction;

// TODO consider alternatative assets
#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum CommanderIcon {
    AtarkCommander,
    KaranCommander,
    NoozlerCommander,
    GranokCommander,
    None,
}

impl Display for CommanderIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommanderIcon::AtarkCommander => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_commander.png"
                )
            }
            CommanderIcon::KaranCommander => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_commander.png"
                )
            }
            CommanderIcon::NoozlerCommander => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_commander.png"
                )
            }
            CommanderIcon::GranokCommander => {
                write!(
                    formatter,
                    "user_interface/icons/starships/granok/granok_commander.png"
                )
            }
            CommanderIcon::None => {
                write!(formatter, "")
            }
        }
    }
}

impl CommanderIcon {
    pub fn convert_from(faction: Faction) -> CommanderIcon {
        match faction {
            Faction::Atark => CommanderIcon::AtarkCommander,
            Faction::Karcan => CommanderIcon::KaranCommander,
            Faction::Noozler => CommanderIcon::NoozlerCommander,
            Faction::Granok => CommanderIcon::GranokCommander,
        }
    }
}

#[cfg(test)]
mod commander_icons_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Faction::Atark, CommanderIcon::AtarkCommander)]
    #[case(Faction::Karcan, CommanderIcon::KaranCommander)]
    #[case(Faction::Noozler, CommanderIcon::NoozlerCommander)]
    #[case(Faction::Granok, CommanderIcon::GranokCommander)]
    fn convert_from(#[case] faction: Faction, #[case] commander_icon: CommanderIcon) {
        // When
        let actual_commander_icon = CommanderIcon::convert_from(faction);

        // Then
        assert_eq!(commander_icon, actual_commander_icon);
    }
}
