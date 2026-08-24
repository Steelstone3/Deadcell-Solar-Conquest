use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum MusicSound {
    #[default]
    MenuMusic,
    CreditMusic,
    Background1,
    Background2,
    Background3,
    Background4,
    Background5,
    AtarkCombat,
    KarcanCombat,
    NoozlerCombat,
}

impl Display for MusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicSound::MenuMusic => {
                write!(formatter, "sounds/music/menu/menu.ogg")
            }
            MusicSound::CreditMusic => {
                write!(formatter, "sounds/music/menu/credits.ogg")
            }
            MusicSound::Background1 => {
                write!(formatter, "sounds/music/background/background_1.ogg")
            }
            MusicSound::Background2 => {
                write!(formatter, "sounds/music/background/background_2.ogg")
            }
            MusicSound::Background3 => {
                write!(formatter, "sounds/music/background/background_3.ogg")
            }
            MusicSound::Background4 => {
                write!(formatter, "sounds/music/background/background_4.ogg")
            }
            MusicSound::Background5 => {
                write!(formatter, "sounds/music/background/background_5.ogg")
            }
            MusicSound::AtarkCombat => {
                write!(formatter, "sounds/music/combat/atark_combat.ogg")
            }
            MusicSound::KarcanCombat => {
                write!(formatter, "sounds/music/combat/karcan_combat.ogg")
            }
            MusicSound::NoozlerCombat => {
                write!(formatter, "sounds/music/combat/noozler_combat.ogg")
            }
        }
    }
}
