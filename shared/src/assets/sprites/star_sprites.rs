use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect, Deserialize, Serialize)]
pub enum StarSprites {
    Star1,
    Star2,
    Star3,
    Star4,
    Star5,
    Star6,
    Star7,
    Star8,
    Star9,
    Star10,
}

impl Display for StarSprites {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarSprites::Star1 => {
                write!(formatter, "images/stars/star_1.png")
            }
            StarSprites::Star2 => {
                write!(formatter, "images/stars/star_2.png")
            }
            StarSprites::Star3 => {
                write!(formatter, "images/stars/star_3.png")
            }
            StarSprites::Star4 => {
                write!(formatter, "images/stars/star_4.png")
            }
            StarSprites::Star5 => {
                write!(formatter, "images/stars/star_5.png")
            }
            StarSprites::Star6 => {
                write!(formatter, "images/stars/star_6.png")
            }
            StarSprites::Star7 => {
                write!(formatter, "images/stars/star_7.png")
            }
            StarSprites::Star8 => {
                write!(formatter, "images/stars/star_8.png")
            }
            StarSprites::Star9 => {
                write!(formatter, "images/stars/star_9.png")
            }
            StarSprites::Star10 => {
                write!(formatter, "images/stars/star_10.png")
            }
        }
    }
}
