use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum WeaponSound {
    #[default]
    Blaster1,
    Blaster2,
    Exotic1,
    Exotic2,
    Exotic3,
    Exotic4,
    Exotic5,
    Exotic6,
    Mine1,
    Mine2,
    Mine3,
    Mine4,
    Mine5,
    Mine6,
    Torpedo1,
    Torpedo2,
    Torpedo3,
    Impact1,
    Impact2,
    Impact3,
    Impact4,
    Impact5,
    Impact6,
    Impact7,
    Impact8,
    Impact9,
    Impact10,
    Impact11,
    Impact12,
}

impl Display for WeaponSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponSound::Blaster1 => {
                write!(formatter, "sounds/starships/weapons/blaster_1.ogg")
            }
            WeaponSound::Blaster2 => {
                write!(formatter, "sounds/starships/weapons/blaster_2.ogg")
            }
            WeaponSound::Exotic1 => write!(formatter, "sounds/starships/weapons/exotic_1.ogg"),
            WeaponSound::Exotic2 => write!(formatter, "sounds/starships/weapons/exotic_2.ogg"),
            WeaponSound::Exotic3 => write!(formatter, "sounds/starships/weapons/exotic_3.ogg"),
            WeaponSound::Exotic4 => write!(formatter, "sounds/starships/weapons/exotic_4.ogg"),
            WeaponSound::Exotic5 => write!(formatter, "sounds/starships/weapons/exotic_5.ogg"),
            WeaponSound::Exotic6 => write!(formatter, "sounds/starships/weapons/exotic_6.ogg"),
            WeaponSound::Mine1 => write!(formatter, "sounds/starships/weapons/mine_1.ogg"),
            WeaponSound::Mine2 => write!(formatter, "sounds/starships/weapons/mine_2.ogg"),
            WeaponSound::Mine3 => write!(formatter, "sounds/starships/weapons/mine_3.ogg"),
            WeaponSound::Mine4 => write!(formatter, "sounds/starships/weapons/mine_4.ogg"),
            WeaponSound::Mine5 => write!(formatter, "sounds/starships/weapons/mine_5.ogg"),
            WeaponSound::Mine6 => write!(formatter, "sounds/starships/weapons/mine_6.ogg"),
            WeaponSound::Torpedo1 => write!(formatter, "sounds/starships/weapons/torpedo_1.ogg"),
            WeaponSound::Torpedo2 => write!(formatter, "sounds/starships/weapons/torpedo_2.ogg"),
            WeaponSound::Torpedo3 => write!(formatter, "sounds/starships/weapons/torpedo_3.ogg"),
            WeaponSound::Impact1 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_1.ogg")
            }
            WeaponSound::Impact2 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_2.ogg")
            }
            WeaponSound::Impact3 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_3.ogg")
            }
            WeaponSound::Impact4 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_4.ogg")
            }
            WeaponSound::Impact5 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_5.ogg")
            }
            WeaponSound::Impact6 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_6.ogg")
            }
            WeaponSound::Impact7 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_7.ogg")
            }
            WeaponSound::Impact8 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_8.ogg")
            }
            WeaponSound::Impact9 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_9.ogg")
            }
            WeaponSound::Impact10 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_10.ogg")
            }
            WeaponSound::Impact11 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_11.ogg")
            }
            WeaponSound::Impact12 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_12.ogg")
            }
        }
    }
}
