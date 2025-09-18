use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect, Deserialize, Serialize)]
pub enum StarSprite {
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

impl Display for StarSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarSprite::Star1 => {
                write!(formatter, "images/stars/star_1.png")
            }
            StarSprite::Star2 => {
                write!(formatter, "images/stars/star_2.png")
            }
            StarSprite::Star3 => {
                write!(formatter, "images/stars/star_3.png")
            }
            StarSprite::Star4 => {
                write!(formatter, "images/stars/star_4.png")
            }
            StarSprite::Star5 => {
                write!(formatter, "images/stars/star_5.png")
            }
            StarSprite::Star6 => {
                write!(formatter, "images/stars/star_6.png")
            }
            StarSprite::Star7 => {
                write!(formatter, "images/stars/star_7.png")
            }
            StarSprite::Star8 => {
                write!(formatter, "images/stars/star_8.png")
            }
            StarSprite::Star9 => {
                write!(formatter, "images/stars/star_9.png")
            }
            StarSprite::Star10 => {
                write!(formatter, "images/stars/star_10.png")
            }
        }
    }
}
