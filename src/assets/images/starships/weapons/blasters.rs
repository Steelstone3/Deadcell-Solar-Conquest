use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum BlasterSprite {
    #[default]
    Blaster1,
    Blaster2,
    Blaster3,
    Blaster4,
    Blaster5,
    Blaster6,
    Blaster7,
    Blaster8,
    Blaster9,
    Blaster10,
    Blaster11,
    Blaster12,
    Blaster13,
    Blaster14,
    Blaster15,
    Blaster16,
    Blaster17,
    Blaster18,
    Blaster19,
    Blaster20,
}

impl Display for BlasterSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlasterSprite::Blaster1 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_1.png")
            }
            BlasterSprite::Blaster2 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_2.png")
            }
            BlasterSprite::Blaster3 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_3.png")
            }
            BlasterSprite::Blaster4 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_4.png")
            }
            BlasterSprite::Blaster5 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_5.png")
            }
            BlasterSprite::Blaster6 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_6.png")
            }
            BlasterSprite::Blaster7 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_7.png")
            }
            BlasterSprite::Blaster8 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_8.png")
            }
            BlasterSprite::Blaster9 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_9.png")
            }
            BlasterSprite::Blaster10 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_10.png"
                )
            }
            BlasterSprite::Blaster11 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_11.png"
                )
            }
            BlasterSprite::Blaster12 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_12.png"
                )
            }
            BlasterSprite::Blaster13 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_13.png"
                )
            }
            BlasterSprite::Blaster14 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_14.png"
                )
            }
            BlasterSprite::Blaster15 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_15.png"
                )
            }
            BlasterSprite::Blaster16 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_16.png"
                )
            }
            BlasterSprite::Blaster17 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_17.png"
                )
            }
            BlasterSprite::Blaster18 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_18.png"
                )
            }
            BlasterSprite::Blaster19 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_19.png"
                )
            }
            BlasterSprite::Blaster20 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_20.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod blaster_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "images/starships/weapons/blasters/blaster_1.png";
        let blaster_sprite = BlasterSprite::default();

        // When
        let file_path = blaster_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        BlasterSprite::Blaster1,
        "images/starships/weapons/blasters/blaster_1.png"
    )]
    #[case(
        BlasterSprite::Blaster2,
        "images/starships/weapons/blasters/blaster_2.png"
    )]
    #[case(
        BlasterSprite::Blaster3,
        "images/starships/weapons/blasters/blaster_3.png"
    )]
    #[case(
        BlasterSprite::Blaster4,
        "images/starships/weapons/blasters/blaster_4.png"
    )]
    #[case(
        BlasterSprite::Blaster5,
        "images/starships/weapons/blasters/blaster_5.png"
    )]
    #[case(
        BlasterSprite::Blaster6,
        "images/starships/weapons/blasters/blaster_6.png"
    )]
    #[case(
        BlasterSprite::Blaster7,
        "images/starships/weapons/blasters/blaster_7.png"
    )]
    #[case(
        BlasterSprite::Blaster8,
        "images/starships/weapons/blasters/blaster_8.png"
    )]
    #[case(
        BlasterSprite::Blaster9,
        "images/starships/weapons/blasters/blaster_9.png"
    )]
    #[case(
        BlasterSprite::Blaster10,
        "images/starships/weapons/blasters/blaster_10.png"
    )]
    #[case(
        BlasterSprite::Blaster11,
        "images/starships/weapons/blasters/blaster_11.png"
    )]
    #[case(
        BlasterSprite::Blaster12,
        "images/starships/weapons/blasters/blaster_12.png"
    )]
    #[case(
        BlasterSprite::Blaster13,
        "images/starships/weapons/blasters/blaster_13.png"
    )]
    #[case(
        BlasterSprite::Blaster14,
        "images/starships/weapons/blasters/blaster_14.png"
    )]
    #[case(
        BlasterSprite::Blaster15,
        "images/starships/weapons/blasters/blaster_15.png"
    )]
    #[case(
        BlasterSprite::Blaster16,
        "images/starships/weapons/blasters/blaster_16.png"
    )]
    #[case(
        BlasterSprite::Blaster17,
        "images/starships/weapons/blasters/blaster_17.png"
    )]
    #[case(
        BlasterSprite::Blaster18,
        "images/starships/weapons/blasters/blaster_18.png"
    )]
    #[case(
        BlasterSprite::Blaster19,
        "images/starships/weapons/blasters/blaster_19.png"
    )]
    #[case(
        BlasterSprite::Blaster20,
        "images/starships/weapons/blasters/blaster_20.png"
    )]
    fn return_the_expected_file_path(
        #[case] blaster_sprite: BlasterSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = blaster_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
