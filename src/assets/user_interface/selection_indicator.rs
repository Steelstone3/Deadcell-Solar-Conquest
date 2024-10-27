use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SelectionIndicator {
    Indicator,
}

impl Display for SelectionIndicator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionIndicator::Indicator => {
                write!(
                    formatter,
                    "user_interface/selection/selection_indicator.png"
                )
            }
        }
    }
}
