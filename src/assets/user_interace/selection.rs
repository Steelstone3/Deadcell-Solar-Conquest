use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SelectionSprite {
    TeamSelect1,
}

impl Display for SelectionSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionSprite::TeamSelect1 => {
                write!(formatter, "user_interface/selection/team_select_1.png")
            }
        }
    }
}
