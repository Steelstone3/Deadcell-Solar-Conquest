use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SunSprite {
    PixelSun,
}

impl Display for SunSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SunSprite::PixelSun => {
                write!(formatter, "images/suns/sun_1.png")
            }
        }
    }
}

#[cfg(test)]
mod sun_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(SunSprite::PixelSun, "images/suns/sun_1.png")]

    fn return_the_expected_file_path(
        #[case] sun_sprite: SunSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = sun_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
