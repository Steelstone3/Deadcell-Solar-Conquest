use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum MineSound {
    #[default]
    Mine1,
    Mine2,
    Mine3,
    Mine4,
    Mine5,
    Mine6,
}

impl Display for MineSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineSound::Mine1 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_1.ogg")
            }
            MineSound::Mine2 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_2.ogg")
            }
            MineSound::Mine3 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_3.ogg")
            }
            MineSound::Mine4 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_4.ogg")
            }
            MineSound::Mine5 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_5.ogg")
            }
            MineSound::Mine6 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_6.ogg")
            }
        }
    }
}

#[cfg(test)]
mod mine_sound_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/starships/weapons/mines/mine_1.ogg";

        // When
        let file_path = MineSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(MineSound::Mine1, "sounds/starships/weapons/mines/mine_1.ogg")]
    #[case(MineSound::Mine2, "sounds/starships/weapons/mines/mine_2.ogg")]
    #[case(MineSound::Mine3, "sounds/starships/weapons/mines/mine_3.ogg")]
    #[case(MineSound::Mine4, "sounds/starships/weapons/mines/mine_4.ogg")]
    #[case(MineSound::Mine5, "sounds/starships/weapons/mines/mine_5.ogg")]
    #[case(MineSound::Mine6, "sounds/starships/weapons/mines/mine_6.ogg")]
    fn return_the_expected_file_path(
        #[case] mine_sound: MineSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = mine_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
