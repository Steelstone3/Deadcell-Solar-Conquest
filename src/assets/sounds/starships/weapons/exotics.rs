use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum ExoticSound {
    #[default]
    Exotic1,
    Exotic2,
    Exotic3,
    Exotic4,
    Exotic5,
    Exotic6,
}

impl Display for ExoticSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExoticSound::Exotic1 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_1.ogg")
            }
            ExoticSound::Exotic2 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_2.ogg")
            }
            ExoticSound::Exotic3 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_3.ogg")
            }
            ExoticSound::Exotic4 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_4.ogg")
            }
            ExoticSound::Exotic5 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_5.ogg")
            }
            ExoticSound::Exotic6 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_6.ogg")
            }
        }
    }
}

#[cfg(test)]
mod exotic_sound_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/starships/weapons/exotics/exotic_1.ogg";

        // When
        let file_path = ExoticSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(ExoticSound::Exotic1, "sounds/starships/weapons/exotics/exotic_1.ogg")]
    #[case(ExoticSound::Exotic2, "sounds/starships/weapons/exotics/exotic_2.ogg")]
    #[case(ExoticSound::Exotic3, "sounds/starships/weapons/exotics/exotic_3.ogg")]
    #[case(ExoticSound::Exotic4, "sounds/starships/weapons/exotics/exotic_4.ogg")]
    #[case(ExoticSound::Exotic5, "sounds/starships/weapons/exotics/exotic_5.ogg")]
    #[case(ExoticSound::Exotic6, "sounds/starships/weapons/exotics/exotic_6.ogg")]
    fn return_the_expected_file_path(
        #[case] exotic_sound: ExoticSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = exotic_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
