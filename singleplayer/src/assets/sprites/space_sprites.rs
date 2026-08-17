use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub enum SpaceSprites {
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

impl Display for SpaceSprites {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceSprites::Space1 => {
                write!(formatter, "images/space/space_1.png")
            }
            SpaceSprites::Space2 => {
                write!(formatter, "images/space/space_2.png")
            }
            SpaceSprites::Space3 => {
                write!(formatter, "images/space/space_3.png")
            }
            SpaceSprites::Space4 => {
                write!(formatter, "images/space/space_4.png")
            }
            SpaceSprites::Space5 => {
                write!(formatter, "images/space/space_5.png")
            }
            SpaceSprites::Space6 => {
                write!(formatter, "images/space/space_6.png")
            }
            SpaceSprites::Space7 => {
                write!(formatter, "images/space/space_7.png")
            }
            SpaceSprites::Space8 => {
                write!(formatter, "images/space/space_8.png")
            }
            SpaceSprites::Space9 => {
                write!(formatter, "images/space/space_9.png")
            }
            SpaceSprites::Space10 => {
                write!(formatter, "images/space/space_10.png")
            }
            SpaceSprites::Space11 => {
                write!(formatter, "images/space/space_11.png")
            }
            SpaceSprites::Space12 => {
                write!(formatter, "images/space/space_12.png")
            }
            SpaceSprites::Space13 => {
                write!(formatter, "images/space/space_13.png")
            }
            SpaceSprites::Space14 => {
                write!(formatter, "images/space/space_14.png")
            }
            SpaceSprites::Space15 => {
                write!(formatter, "images/space/space_15.png")
            }
            SpaceSprites::Space16 => {
                write!(formatter, "images/space/space_16.png")
            }
            SpaceSprites::Space17 => {
                write!(formatter, "images/space/space_17.png")
            }
            SpaceSprites::Space18 => {
                write!(formatter, "images/space/space_18.png")
            }
            SpaceSprites::Space19 => {
                write!(formatter, "images/space/space_19.png")
            }
            SpaceSprites::Space20 => {
                write!(formatter, "images/space/space_20.png")
            }
        }
    }
}
