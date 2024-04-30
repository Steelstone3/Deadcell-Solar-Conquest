use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum TorpedoSprite {
    #[default]
    Torpedo1,
    Torpedo2,
    Torpedo3,
    Torpedo4,
    Torpedo5,
    Torpedo6,
    Torpedo7,
    Torpedo8,
    Torpedo9,
}

impl Display for TorpedoSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TorpedoSprite::Torpedo1 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_1.png"
                )
            }
            TorpedoSprite::Torpedo2 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_2.png"
                )
            }
            TorpedoSprite::Torpedo3 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_3.png"
                )
            }
            TorpedoSprite::Torpedo4 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_4.png"
                )
            }
            TorpedoSprite::Torpedo5 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_5.png"
                )
            }
            TorpedoSprite::Torpedo6 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_6.png"
                )
            }
            TorpedoSprite::Torpedo7 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_7.png"
                )
            }
            TorpedoSprite::Torpedo8 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_8.png"
                )
            }
            TorpedoSprite::Torpedo9 => {
                write!(
                    formatter,
                    "images/starships/weapons/torpedoes/torpedo_9.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod torpedo_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "images/starships/weapons/torpedoes/torpedo_1.png";
        let torpedo_sprite = TorpedoSprite::default();

        // When
        let file_path = torpedo_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        TorpedoSprite::Torpedo1,
        "images/starships/weapons/torpedoes/torpedo_1.png"
    )]
    #[case(
        TorpedoSprite::Torpedo2,
        "images/starships/weapons/torpedoes/torpedo_2.png"
    )]
    #[case(
        TorpedoSprite::Torpedo3,
        "images/starships/weapons/torpedoes/torpedo_3.png"
    )]
    #[case(
        TorpedoSprite::Torpedo4,
        "images/starships/weapons/torpedoes/torpedo_4.png"
    )]
    #[case(
        TorpedoSprite::Torpedo5,
        "images/starships/weapons/torpedoes/torpedo_5.png"
    )]
    #[case(
        TorpedoSprite::Torpedo6,
        "images/starships/weapons/torpedoes/torpedo_6.png"
    )]
    #[case(
        TorpedoSprite::Torpedo7,
        "images/starships/weapons/torpedoes/torpedo_7.png"
    )]
    #[case(
        TorpedoSprite::Torpedo8,
        "images/starships/weapons/torpedoes/torpedo_8.png"
    )]
    #[case(
        TorpedoSprite::Torpedo9,
        "images/starships/weapons/torpedoes/torpedo_9.png"
    )]
    fn return_the_expected_file_path(
        #[case] torpedo_sprite: TorpedoSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = torpedo_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
