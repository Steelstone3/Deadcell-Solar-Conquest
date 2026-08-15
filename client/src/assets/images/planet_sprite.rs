use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect, Deserialize, Serialize)]
pub enum PlanetSprite {
    AridWorld1,
    AridWorld2,
    AridWorld3,
    AridWorld4,
    AridWorld5,
    AridWorld6,
    AridWorld7,
    AridWorld8,
    AridWorld9,
    AridWorld10,
    BarrenWorld1,
    BarrenWorld2,
    BarrenWorld3,
    BarrenWorld4,
    BarrenWorld5,
    BarrenWorld6,
    BarrenWorld7,
    BarrenWorld8,
    BarrenWorld9,
    BarrenWorld10,
    GasGiantWorld1,
    GasGiantWorld2,
    GasGiantWorld3,
    GasGiantWorld4,
    GasGiantWorld5,
    // GasGiantWorld6,
    // GasGiantWorld7,
    // GasGiantWorld8,
    // GasGiantWorld9,
    // GasGiantWorld10,
    IceWorld1,
    IceWorld2,
    IceWorld3,
    IceWorld4,
    IceWorld5,
    IceWorld6,
    IceWorld7,
    IceWorld8,
    IceWorld9,
    IceWorld10,
    ParadiseWorld1,
    ParadiseWorld2,
    ParadiseWorld3,
    ParadiseWorld4,
    ParadiseWorld5,
    ParadiseWorld6,
    ParadiseWorld7,
    ParadiseWorld8,
    ParadiseWorld9,
    ParadiseWorld10,
    WetWorld1,
    WetWorld2,
    WetWorld3,
    WetWorld4,
    WetWorld5,
    WetWorld6,
    WetWorld7,
    WetWorld8,
    WetWorld9,
    WetWorld10,
    // BlackHole1,
    // BlackHole2,
    // BlackHole3,
    // BlackHole4,
    // BlackHole5,
    // Galaxy1,
    // Galaxy2,
    // Galaxy3,
    // Galaxy4,
    // Galaxy5,
}

impl Display for PlanetSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetSprite::AridWorld1 => {
                write!(formatter, "images/planets/arid_world_1.png")
            }
            PlanetSprite::AridWorld2 => {
                write!(formatter, "images/planets/arid_world_2.png")
            }
            PlanetSprite::AridWorld3 => {
                write!(formatter, "images/planets/arid_world_3.png")
            }
            PlanetSprite::AridWorld4 => {
                write!(formatter, "images/planets/arid_world_4.png")
            }
            PlanetSprite::AridWorld5 => {
                write!(formatter, "images/planets/arid_world_5.png")
            }
            PlanetSprite::AridWorld6 => {
                write!(formatter, "images/planets/arid_world_6.png")
            }
            PlanetSprite::AridWorld7 => {
                write!(formatter, "images/planets/arid_world_7.png")
            }
            PlanetSprite::AridWorld8 => {
                write!(formatter, "images/planets/arid_world_8.png")
            }
            PlanetSprite::AridWorld9 => {
                write!(formatter, "images/planets/arid_world_9.png")
            }
            PlanetSprite::AridWorld10 => {
                write!(formatter, "images/planets/arid_world_10.png")
            }
            PlanetSprite::BarrenWorld1 => {
                write!(formatter, "images/planets/barren_world_1.png")
            }
            PlanetSprite::BarrenWorld2 => {
                write!(formatter, "images/planets/barren_world_2.png")
            }
            PlanetSprite::BarrenWorld3 => {
                write!(formatter, "images/planets/barren_world_3.png")
            }
            PlanetSprite::BarrenWorld4 => {
                write!(formatter, "images/planets/barren_world_4.png")
            }
            PlanetSprite::BarrenWorld5 => {
                write!(formatter, "images/planets/barren_world_5.png")
            }
            PlanetSprite::BarrenWorld6 => {
                write!(formatter, "images/planets/barren_world_6.png")
            }
            PlanetSprite::BarrenWorld7 => {
                write!(formatter, "images/planets/barren_world_7.png")
            }
            PlanetSprite::BarrenWorld8 => {
                write!(formatter, "images/planets/barren_world_8.png")
            }
            PlanetSprite::BarrenWorld9 => {
                write!(formatter, "images/planets/barren_world_9.png")
            }
            PlanetSprite::BarrenWorld10 => {
                write!(formatter, "images/planets/barren_world_10.png")
            }
            PlanetSprite::GasGiantWorld1 => {
                write!(formatter, "images/planets/gas_giant_world_1.png")
            }
            PlanetSprite::GasGiantWorld2 => {
                write!(formatter, "images/planets/gas_giant_world_2.png")
            }
            PlanetSprite::GasGiantWorld3 => {
                write!(formatter, "images/planets/gas_giant_world_3.png")
            }
            PlanetSprite::GasGiantWorld4 => {
                write!(formatter, "images/planets/gas_giant_world_4.png")
            }
            PlanetSprite::GasGiantWorld5 => {
                write!(formatter, "images/planets/gas_giant_world_5.png")
            }
            // PlanetSprite::GasGiantWorld6 => {
            //     write!(formatter, "images/planets/gas_giant_world_6.png")
            // }
            // PlanetSprite::GasGiantWorld7 => {
            //     write!(formatter, "images/planets/gas_giant_world_7.png")
            // }
            // PlanetSprite::GasGiantWorld8 => {
            //     write!(formatter, "images/planets/gas_giant_world_8.png")
            // }
            // PlanetSprite::GasGiantWorld9 => {
            //     write!(formatter, "images/planets/gas_giant_world_9.png")
            // }
            // PlanetSprite::GasGiantWorld10 => {
            //     write!(formatter, "images/planets/gas_giant_world_10.png")
            // }
            PlanetSprite::IceWorld1 => {
                write!(formatter, "images/planets/ice_world_1.png")
            }
            PlanetSprite::IceWorld2 => {
                write!(formatter, "images/planets/ice_world_2.png")
            }
            PlanetSprite::IceWorld3 => {
                write!(formatter, "images/planets/ice_world_3.png")
            }
            PlanetSprite::IceWorld4 => {
                write!(formatter, "images/planets/ice_world_4.png")
            }
            PlanetSprite::IceWorld5 => {
                write!(formatter, "images/planets/ice_world_5.png")
            }
            PlanetSprite::IceWorld6 => {
                write!(formatter, "images/planets/ice_world_6.png")
            }
            PlanetSprite::IceWorld7 => {
                write!(formatter, "images/planets/ice_world_7.png")
            }
            PlanetSprite::IceWorld8 => {
                write!(formatter, "images/planets/ice_world_8.png")
            }
            PlanetSprite::IceWorld9 => {
                write!(formatter, "images/planets/ice_world_9.png")
            }
            PlanetSprite::IceWorld10 => {
                write!(formatter, "images/planets/ice_world_10.png")
            }
            PlanetSprite::ParadiseWorld1 => {
                write!(formatter, "images/planets/paradise_world_1.png")
            }
            PlanetSprite::ParadiseWorld2 => {
                write!(formatter, "images/planets/paradise_world_2.png")
            }
            PlanetSprite::ParadiseWorld3 => {
                write!(formatter, "images/planets/paradise_world_3.png")
            }
            PlanetSprite::ParadiseWorld4 => {
                write!(formatter, "images/planets/paradise_world_4.png")
            }
            PlanetSprite::ParadiseWorld5 => {
                write!(formatter, "images/planets/paradise_world_5.png")
            }
            PlanetSprite::ParadiseWorld6 => {
                write!(formatter, "images/planets/paradise_world_6.png")
            }
            PlanetSprite::ParadiseWorld7 => {
                write!(formatter, "images/planets/paradise_world_7.png")
            }
            PlanetSprite::ParadiseWorld8 => {
                write!(formatter, "images/planets/paradise_world_8.png")
            }
            PlanetSprite::ParadiseWorld9 => {
                write!(formatter, "images/planets/paradise_world_9.png")
            }
            PlanetSprite::ParadiseWorld10 => {
                write!(formatter, "images/planets/paradise_world_10.png")
            }
            PlanetSprite::WetWorld1 => {
                write!(formatter, "images/planets/wet_world_1.png")
            }
            PlanetSprite::WetWorld2 => {
                write!(formatter, "images/planets/wet_world_2.png")
            }
            PlanetSprite::WetWorld3 => {
                write!(formatter, "images/planets/wet_world_3.png")
            }
            PlanetSprite::WetWorld4 => {
                write!(formatter, "images/planets/wet_world_4.png")
            }
            PlanetSprite::WetWorld5 => {
                write!(formatter, "images/planets/wet_world_5.png")
            }
            PlanetSprite::WetWorld6 => {
                write!(formatter, "images/planets/wet_world_6.png")
            }
            PlanetSprite::WetWorld7 => {
                write!(formatter, "images/planets/wet_world_7.png")
            }
            PlanetSprite::WetWorld8 => {
                write!(formatter, "images/planets/wet_world_8.png")
            }
            PlanetSprite::WetWorld9 => {
                write!(formatter, "images/planets/wet_world_9.png")
            }
            PlanetSprite::WetWorld10 => {
                write!(formatter, "images/planets/wet_world_10.png")
            } // PlanetSprite::BlackHole1 => {
              //     write!(formatter, "images/planets/black_hole_1.png")
              // }
              // PlanetSprite::BlackHole2 => {
              //     write!(formatter, "images/planets/black_hole_2.png")
              // }
              // PlanetSprite::BlackHole3 => {
              //     write!(formatter, "images/planets/black_hole_3.png")
              // }
              // PlanetSprite::BlackHole4 => {
              //     write!(formatter, "images/planets/black_hole_4.png")
              // }
              // PlanetSprite::BlackHole5 => {
              //     write!(formatter, "images/planets/black_hole_5.png")
              // }
              // PlanetSprite::Galaxy1 => {
              //     write!(formatter, "images/planets/galaxy_1.png")
              // }
              // PlanetSprite::Galaxy2 => {
              //     write!(formatter, "images/planets/galaxy_2.png")
              // }
              // PlanetSprite::Galaxy3 => {
              //     write!(formatter, "images/planets/galaxy_3.png")
              // }
              // PlanetSprite::Galaxy4 => {
              //     write!(formatter, "images/planets/galaxy_4.png")
              // }
              // PlanetSprite::Galaxy5 => {
              //     write!(formatter, "images/planets/galaxy_5.png")
              // }
        }
    }
}
