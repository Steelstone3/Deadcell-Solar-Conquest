use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect)]
pub enum SunSprite {
    Sun1,
    Sun2,
    Sun3,
    Sun4,
    Sun5,
    Sun6,
    Sun7,
    Sun8,
    Sun9,
    Sun10,
}

impl Display for SunSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SunSprite::Sun1 => {
                write!(formatter, "images/suns/sun_1.png")
            }
            SunSprite::Sun2 => {
                write!(formatter, "images/suns/sun_2.png")
            }
            SunSprite::Sun3 => {
                write!(formatter, "images/suns/sun_3.png")
            }
            SunSprite::Sun4 => {
                write!(formatter, "images/suns/sun_4.png")
            }
            SunSprite::Sun5 => {
                write!(formatter, "images/suns/sun_5.png")
            }
            SunSprite::Sun6 => {
                write!(formatter, "images/suns/sun_6.png")
            }
            SunSprite::Sun7 => {
                write!(formatter, "images/suns/sun_7.png")
            }
            SunSprite::Sun8 => {
                write!(formatter, "images/suns/sun_8.png")
            }
            SunSprite::Sun9 => {
                write!(formatter, "images/suns/sun_9.png")
            }
            SunSprite::Sun10 => {
                write!(formatter, "images/suns/sun_10.png")
            }
        }
    }
}
