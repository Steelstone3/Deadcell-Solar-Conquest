use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum TorpedoSound {
    #[default]
    Torpedo1,
    Torpedo2,
    Torpedo3,
}

impl Display for TorpedoSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TorpedoSound::Torpedo1 => {
                write!(formatter, "sounds/starships/weapons/torpedos/torpedo_1.ogg")
            }
            TorpedoSound::Torpedo2 => {
                write!(formatter, "sounds/starships/weapons/torpedos/torpedo_2.ogg")
            }
            TorpedoSound::Torpedo3 => {
                write!(formatter, "sounds/starships/weapons/torpedos/torpedo_3.ogg")
            }
        }
    }
}

#[cfg(test)]
mod torpedo_sound_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/starships/weapons/torpedos/torpedo_1.ogg";

        // When
        let file_path = TorpedoSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        TorpedoSound::Torpedo1,
        "sounds/starships/weapons/torpedos/torpedo_1.ogg"
    )]
    #[case(
        TorpedoSound::Torpedo2,
        "sounds/starships/weapons/torpedos/torpedo_2.ogg"
    )]
    #[case(
        TorpedoSound::Torpedo3,
        "sounds/starships/weapons/torpedos/torpedo_3.ogg"
    )]
    fn return_the_expected_file_path(
        #[case] torpedo_sound: TorpedoSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = torpedo_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
