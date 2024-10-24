use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum MultiSelectSprite {
    MultiSelectNeutral,
}

impl Display for MultiSelectSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiSelectSprite::MultiSelectNeutral => {
                write!(formatter, "user_interface/selection/multiselect.png")
            }
        }
    }
}
