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
