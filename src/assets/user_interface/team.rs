use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum TeamSelectionSprite {
    Team1,
}

impl Display for TeamSelectionSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamSelectionSprite::Team1 => {
                write!(formatter, "user_interface/selection/team_select_1.png")
            }
        }
    }
}
