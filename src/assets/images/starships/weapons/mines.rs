use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum MineSprite {
    #[default]
    Mine1,
    Mine2,
    Mine3,
    Mine4,
    Mine5,
    Mine6,
    Mine7,
    Mine8,
    Mine9,
}

impl Display for MineSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineSprite::Mine1 => {
                write!(formatter, "images/starships/weapons/mines/mine_1.png")
            }
            MineSprite::Mine2 => {
                write!(formatter, "images/starships/weapons/mines/mine_2.png")
            }
            MineSprite::Mine3 => {
                write!(formatter, "images/starships/weapons/mines/mine_3.png")
            }
            MineSprite::Mine4 => {
                write!(formatter, "images/starships/weapons/mines/mine_4.png")
            }
            MineSprite::Mine5 => {
                write!(formatter, "images/starships/weapons/mines/mine_5.png")
            }
            MineSprite::Mine6 => {
                write!(formatter, "images/starships/weapons/mines/mine_6.png")
            }
            MineSprite::Mine7 => {
                write!(formatter, "images/starships/weapons/mines/mine_7.png")
            }
            MineSprite::Mine8 => {
                write!(formatter, "images/starships/weapons/mines/mine_8.png")
            }
            MineSprite::Mine9 => {
                write!(formatter, "images/starships/weapons/mines/mine_9.png")
            }
        }
    }
}

#[cfg(test)]
mod mine_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "images/starships/weapons/mines/mine_1.png";
        let mine_sprite = MineSprite::default();

        // When
        let file_path = mine_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(MineSprite::Mine1, "images/starships/weapons/mines/mine_1.png")]
    #[case(MineSprite::Mine2, "images/starships/weapons/mines/mine_2.png")]
    #[case(MineSprite::Mine3, "images/starships/weapons/mines/mine_3.png")]
    #[case(MineSprite::Mine4, "images/starships/weapons/mines/mine_4.png")]
    #[case(MineSprite::Mine5, "images/starships/weapons/mines/mine_5.png")]
    #[case(MineSprite::Mine6, "images/starships/weapons/mines/mine_6.png")]
    #[case(MineSprite::Mine7, "images/starships/weapons/mines/mine_7.png")]
    #[case(MineSprite::Mine8, "images/starships/weapons/mines/mine_8.png")]
    #[case(MineSprite::Mine9, "images/starships/weapons/mines/mine_9.png")]
    fn return_the_expected_file_path(
        #[case] mine_sprite: MineSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = mine_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
