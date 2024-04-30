use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum ExoticSprite {
    #[default]
    Exotic1,
    Exotic2,
    Exotic3,
    Exotic4,
    Exotic5,
    Exotic6,
    Exotic7,
    Exotic8,
    Exotic9,
    Exotic10,
    Exotic11,
    Exotic12,
    Exotic13,
    Exotic14,
    Exotic15,
    Exotic16,
    Exotic17,
    Exotic18,
    Exotic19,
    Exotic20,
    Exotic21,
    Exotic22,
    Exotic23,
    Exotic24,
    Exotic25,
    Exotic26,
    Exotic27,
    Exotic28,
}

impl Display for ExoticSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExoticSprite::Exotic1 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_1.png")
            }
            ExoticSprite::Exotic2 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_2.png")
            }
            ExoticSprite::Exotic3 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_3.png")
            }
            ExoticSprite::Exotic4 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_4.png")
            }
            ExoticSprite::Exotic5 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_5.png")
            }
            ExoticSprite::Exotic6 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_6.png")
            }
            ExoticSprite::Exotic7 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_7.png")
            }
            ExoticSprite::Exotic8 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_8.png")
            }
            ExoticSprite::Exotic9 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_9.png")
            }
            ExoticSprite::Exotic10 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_10.png")
            }
            ExoticSprite::Exotic11 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_11.png")
            }
            ExoticSprite::Exotic12 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_12.png")
            }
            ExoticSprite::Exotic13 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_13.png")
            }
            ExoticSprite::Exotic14 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_14.png")
            }
            ExoticSprite::Exotic15 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_15.png")
            }
            ExoticSprite::Exotic16 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_16.png")
            }
            ExoticSprite::Exotic17 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_17.png")
            }
            ExoticSprite::Exotic18 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_18.png")
            }
            ExoticSprite::Exotic19 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_19.png")
            }
            ExoticSprite::Exotic20 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_20.png")
            }
            ExoticSprite::Exotic21 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_21.png")
            }
            ExoticSprite::Exotic22 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_22.png")
            }
            ExoticSprite::Exotic23 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_23.png")
            }
            ExoticSprite::Exotic24 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_24.png")
            }
            ExoticSprite::Exotic25 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_25.png")
            }
            ExoticSprite::Exotic26 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_26.png")
            }
            ExoticSprite::Exotic27 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_27.png")
            }
            ExoticSprite::Exotic28 => {
                write!(formatter, "images/starships/weapons/exotics/exotic_28.png")
            }
        }
    }
}

#[cfg(test)]
mod exotic_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "images/starships/weapons/exotics/exotic_1.png";
        let exotic_sprite = ExoticSprite::default();

        // When
        let file_path = exotic_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(ExoticSprite::Exotic1, "images/starships/weapons/exotics/exotic_1.png")]
    #[case(ExoticSprite::Exotic2, "images/starships/weapons/exotics/exotic_2.png")]
    #[case(ExoticSprite::Exotic3, "images/starships/weapons/exotics/exotic_3.png")]
    #[case(ExoticSprite::Exotic4, "images/starships/weapons/exotics/exotic_4.png")]
    #[case(ExoticSprite::Exotic5, "images/starships/weapons/exotics/exotic_5.png")]
    #[case(ExoticSprite::Exotic6, "images/starships/weapons/exotics/exotic_6.png")]
    #[case(ExoticSprite::Exotic7, "images/starships/weapons/exotics/exotic_7.png")]
    #[case(ExoticSprite::Exotic8, "images/starships/weapons/exotics/exotic_8.png")]
    #[case(ExoticSprite::Exotic9, "images/starships/weapons/exotics/exotic_9.png")]
    #[case(
        ExoticSprite::Exotic10,
        "images/starships/weapons/exotics/exotic_10.png"
    )]
    #[case(
        ExoticSprite::Exotic11,
        "images/starships/weapons/exotics/exotic_11.png"
    )]
    #[case(
        ExoticSprite::Exotic12,
        "images/starships/weapons/exotics/exotic_12.png"
    )]
    #[case(
        ExoticSprite::Exotic13,
        "images/starships/weapons/exotics/exotic_13.png"
    )]
    #[case(
        ExoticSprite::Exotic14,
        "images/starships/weapons/exotics/exotic_14.png"
    )]
    #[case(
        ExoticSprite::Exotic15,
        "images/starships/weapons/exotics/exotic_15.png"
    )]
    #[case(
        ExoticSprite::Exotic16,
        "images/starships/weapons/exotics/exotic_16.png"
    )]
    #[case(
        ExoticSprite::Exotic17,
        "images/starships/weapons/exotics/exotic_17.png"
    )]
    #[case(
        ExoticSprite::Exotic18,
        "images/starships/weapons/exotics/exotic_18.png"
    )]
    #[case(
        ExoticSprite::Exotic19,
        "images/starships/weapons/exotics/exotic_19.png"
    )]
    #[case(
        ExoticSprite::Exotic20,
        "images/starships/weapons/exotics/exotic_20.png"
    )]
    #[case(
        ExoticSprite::Exotic21,
        "images/starships/weapons/exotics/exotic_21.png"
    )]
    #[case(
        ExoticSprite::Exotic22,
        "images/starships/weapons/exotics/exotic_22.png"
    )]
    #[case(
        ExoticSprite::Exotic23,
        "images/starships/weapons/exotics/exotic_23.png"
    )]
    #[case(
        ExoticSprite::Exotic24,
        "images/starships/weapons/exotics/exotic_24.png"
    )]
    #[case(
        ExoticSprite::Exotic25,
        "images/starships/weapons/exotics/exotic_25.png"
    )]
    #[case(
        ExoticSprite::Exotic26,
        "images/starships/weapons/exotics/exotic_26.png"
    )]
    #[case(
        ExoticSprite::Exotic27,
        "images/starships/weapons/exotics/exotic_27.png"
    )]
    #[case(
        ExoticSprite::Exotic28,
        "images/starships/weapons/exotics/exotic_28.png"
    )]
    fn return_the_expected_file_path(
        #[case] exotic_sprite: ExoticSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = exotic_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
