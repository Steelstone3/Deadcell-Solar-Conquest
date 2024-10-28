use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub enum SpaceSprite {
    Space1,
    Space2,
    Space3,
    Space4,
    Space5,
    Space6,
    Space7,
    Space8,
    Space9,
    Space10,
    Space11,
    Space12,
    Space13,
    Space14,
    Space15,
    Space16,
    Space17,
    Space18,
    Space19,
    Space20,
}

impl Display for SpaceSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceSprite::Space1 => {
                write!(formatter, "images/space/space_1.png")
            }
            SpaceSprite::Space2 => {
                write!(formatter, "images/space/space_2.png")
            }
            SpaceSprite::Space3 => {
                write!(formatter, "images/space/space_3.png")
            }
            SpaceSprite::Space4 => {
                write!(formatter, "images/space/space_4.png")
            }
            SpaceSprite::Space5 => {
                write!(formatter, "images/space/space_5.png")
            }
            SpaceSprite::Space6 => {
                write!(formatter, "images/space/space_6.png")
            }
            SpaceSprite::Space7 => {
                write!(formatter, "images/space/space_7.png")
            }
            SpaceSprite::Space8 => {
                write!(formatter, "images/space/space_8.png")
            }
            SpaceSprite::Space9 => {
                write!(formatter, "images/space/space_9.png")
            }
            SpaceSprite::Space10 => {
                write!(formatter, "images/space/space_10.png")
            }
            SpaceSprite::Space11 => {
                write!(formatter, "images/space/space_11.png")
            }
            SpaceSprite::Space12 => {
                write!(formatter, "images/space/space_12.png")
            }
            SpaceSprite::Space13 => {
                write!(formatter, "images/space/space_13.png")
            }
            SpaceSprite::Space14 => {
                write!(formatter, "images/space/space_14.png")
            }
            SpaceSprite::Space15 => {
                write!(formatter, "images/space/space_15.png")
            }
            SpaceSprite::Space16 => {
                write!(formatter, "images/space/space_16.png")
            }
            SpaceSprite::Space17 => {
                write!(formatter, "images/space/space_17.png")
            }
            SpaceSprite::Space18 => {
                write!(formatter, "images/space/space_18.png")
            }
            SpaceSprite::Space19 => {
                write!(formatter, "images/space/space_19.png")
            }
            SpaceSprite::Space20 => {
                write!(formatter, "images/space/space_20.png")
            }
        }
    }
}
