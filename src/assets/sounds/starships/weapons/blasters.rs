use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum BlasterSound {
    #[default]
    Blaster1,
    Blaster2,
}

impl Display for BlasterSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlasterSound::Blaster1 => {
                write!(formatter, "sounds/starships/weapons/blasters/blaster_1.ogg")
            }
            BlasterSound::Blaster2 => {
                write!(formatter, "sounds/starships/weapons/blasters/blaster_2.ogg")
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/starships/weapons/blasters/blaster_1.ogg";

        // When
        let file_path = BlasterSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        BlasterSound::Blaster1,
        "sounds/starships/weapons/blasters/blaster_1.ogg"
    )]
    #[case(
        BlasterSound::Blaster2,
        "sounds/starships/weapons/blasters/blaster_2.ogg"
    )]
    fn return_the_expected_file_path(
        #[case] blaster_sound: BlasterSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = blaster_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
