use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum EngineSound {
    #[default]
    Engine1,
    Engine2,
    Engine3,
    Engine4,
    Engine5,
    Engine6,
    Engine7,
    Engine8,
}

impl Display for EngineSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineSound::Engine1 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_1.ogg"
                )
            }
            EngineSound::Engine2 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_2.ogg"
                )
            }
            EngineSound::Engine3 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_3.ogg"
                )
            }
            EngineSound::Engine4 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_4.ogg"
                )
            }
            EngineSound::Engine5 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_5.ogg"
                )
            }
            EngineSound::Engine6 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_6.ogg"
                )
            }
            EngineSound::Engine7 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_7.ogg"
                )
            }
            EngineSound::Engine8 => {
                write!(
                    formatter,
                    "sounds/starships/engines/rumble/engine_rumble_8.ogg"
                )
            }
        }
    }
}

#[cfg(test)]
mod engine_sound_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/starships/engines/rumble/engine_rumble_1.ogg";
        let engine_sound = EngineSound::default();

        // When
        let file_path = engine_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        EngineSound::Engine1,
        "sounds/starships/engines/rumble/engine_rumble_1.ogg"
    )]
    #[case(
        EngineSound::Engine2,
        "sounds/starships/engines/rumble/engine_rumble_2.ogg"
    )]
    #[case(
        EngineSound::Engine3,
        "sounds/starships/engines/rumble/engine_rumble_3.ogg"
    )]
    #[case(
        EngineSound::Engine4,
        "sounds/starships/engines/rumble/engine_rumble_4.ogg"
    )]
    #[case(
        EngineSound::Engine5,
        "sounds/starships/engines/rumble/engine_rumble_5.ogg"
    )]
    #[case(
        EngineSound::Engine6,
        "sounds/starships/engines/rumble/engine_rumble_6.ogg"
    )]
    #[case(
        EngineSound::Engine7,
        "sounds/starships/engines/rumble/engine_rumble_7.ogg"
    )]
    #[case(
        EngineSound::Engine8,
        "sounds/starships/engines/rumble/engine_rumble_8.ogg"
    )]
    fn return_the_expected_file_path(
        #[case] engine_sound: EngineSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = engine_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
