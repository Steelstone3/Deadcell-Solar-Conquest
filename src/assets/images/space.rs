use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect)]
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
    Space21,
    Space22,
    Space23,
    Space24,
    Space25,
    Space26,
    Space27,
    Space28,
    Space29,
    Space30,
    Space31,
    Space32,
}

impl Display for SpaceSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceSprite::Space1 => {
                write!(formatter, "images/space/space/space_1.png")
            }
            SpaceSprite::Space2 => {
                write!(formatter, "images/space/space/space_2.png")
            }
            SpaceSprite::Space3 => {
                write!(formatter, "images/space/space/space_3.png")
            }
            SpaceSprite::Space4 => {
                write!(formatter, "images/space/space/space_4.png")
            }
            SpaceSprite::Space5 => {
                write!(formatter, "images/space/space/space_5.png")
            }
            SpaceSprite::Space6 => {
                write!(formatter, "images/space/space/space_6.png")
            }
            SpaceSprite::Space7 => {
                write!(formatter, "images/space/space/space_7.png")
            }
            SpaceSprite::Space8 => {
                write!(formatter, "images/space/space/space_8.png")
            }
            SpaceSprite::Space9 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_1.png")
            }
            SpaceSprite::Space10 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_2.png")
            }
            SpaceSprite::Space11 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_3.png")
            }
            SpaceSprite::Space12 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_4.png")
            }
            SpaceSprite::Space13 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_5.png")
            }
            SpaceSprite::Space14 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_6.png")
            }
            SpaceSprite::Space15 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_7.png")
            }
            SpaceSprite::Space16 => {
                write!(formatter, "images/space/purple_nebula/purple_nebula_8.png")
            }
            SpaceSprite::Space17 => {
                write!(formatter, "images/space/green_nebula/green_nebula_1.png")
            }
            SpaceSprite::Space18 => {
                write!(formatter, "images/space/green_nebula/green_nebula_2.png")
            }
            SpaceSprite::Space19 => {
                write!(formatter, "images/space/green_nebula/green_nebula_3.png")
            }
            SpaceSprite::Space20 => {
                write!(formatter, "images/space/green_nebula/green_nebula_4.png")
            }
            SpaceSprite::Space21 => {
                write!(formatter, "images/space/green_nebula/green_nebula_5.png")
            }
            SpaceSprite::Space22 => {
                write!(formatter, "images/space/green_nebula/green_nebula_6.png")
            }
            SpaceSprite::Space23 => {
                write!(formatter, "images/space/green_nebula/green_nebula_7.png")
            }
            SpaceSprite::Space24 => {
                write!(formatter, "images/space/green_nebula/green_nebula_8.png")
            }
            SpaceSprite::Space25 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_1.png")
            }
            SpaceSprite::Space26 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_2.png")
            }
            SpaceSprite::Space27 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_3.png")
            }
            SpaceSprite::Space28 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_4.png")
            }
            SpaceSprite::Space29 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_5.png")
            }
            SpaceSprite::Space30 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_6.png")
            }
            SpaceSprite::Space31 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_7.png")
            }
            SpaceSprite::Space32 => {
                write!(formatter, "images/space/blue_nebula/blue_nebula_8.png")
            }
        }
    }
}

#[cfg(test)]
mod space_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(SpaceSprite::Space1, "images/space/space/space_1.png")]
    #[case(SpaceSprite::Space2, "images/space/space/space_2.png")]
    #[case(SpaceSprite::Space3, "images/space/space/space_3.png")]
    #[case(SpaceSprite::Space4, "images/space/space/space_4.png")]
    #[case(SpaceSprite::Space5, "images/space/space/space_5.png")]
    #[case(SpaceSprite::Space6, "images/space/space/space_6.png")]
    #[case(SpaceSprite::Space7, "images/space/space/space_7.png")]
    #[case(SpaceSprite::Space8, "images/space/space/space_8.png")]
    #[case(SpaceSprite::Space9, "images/space/purple_nebula/purple_nebula_1.png")]
    #[case(SpaceSprite::Space10, "images/space/purple_nebula/purple_nebula_2.png")]
    #[case(SpaceSprite::Space11, "images/space/purple_nebula/purple_nebula_3.png")]
    #[case(SpaceSprite::Space12, "images/space/purple_nebula/purple_nebula_4.png")]
    #[case(SpaceSprite::Space13, "images/space/purple_nebula/purple_nebula_5.png")]
    #[case(SpaceSprite::Space14, "images/space/purple_nebula/purple_nebula_6.png")]
    #[case(SpaceSprite::Space15, "images/space/purple_nebula/purple_nebula_7.png")]
    #[case(SpaceSprite::Space16, "images/space/purple_nebula/purple_nebula_8.png")]
    #[case(SpaceSprite::Space17, "images/space/green_nebula/green_nebula_1.png")]
    #[case(SpaceSprite::Space18, "images/space/green_nebula/green_nebula_2.png")]
    #[case(SpaceSprite::Space19, "images/space/green_nebula/green_nebula_3.png")]
    #[case(SpaceSprite::Space20, "images/space/green_nebula/green_nebula_4.png")]
    #[case(SpaceSprite::Space21, "images/space/green_nebula/green_nebula_5.png")]
    #[case(SpaceSprite::Space22, "images/space/green_nebula/green_nebula_6.png")]
    #[case(SpaceSprite::Space23, "images/space/green_nebula/green_nebula_7.png")]
    #[case(SpaceSprite::Space24, "images/space/green_nebula/green_nebula_8.png")]
    #[case(SpaceSprite::Space25, "images/space/blue_nebula/blue_nebula_1.png")]
    #[case(SpaceSprite::Space26, "images/space/blue_nebula/blue_nebula_2.png")]
    #[case(SpaceSprite::Space27, "images/space/blue_nebula/blue_nebula_3.png")]
    #[case(SpaceSprite::Space28, "images/space/blue_nebula/blue_nebula_4.png")]
    #[case(SpaceSprite::Space29, "images/space/blue_nebula/blue_nebula_5.png")]
    #[case(SpaceSprite::Space30, "images/space/blue_nebula/blue_nebula_6.png")]
    #[case(SpaceSprite::Space31, "images/space/blue_nebula/blue_nebula_7.png")]
    #[case(SpaceSprite::Space32, "images/space/blue_nebula/blue_nebula_8.png")]
    fn return_the_expected_file_path(
        #[case] space_sprite: SpaceSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = space_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
