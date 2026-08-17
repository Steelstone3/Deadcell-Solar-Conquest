use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect, Deserialize, Serialize)]
pub enum PlanetSprites {
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

impl Display for PlanetSprites {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetSprites::AridWorld1 => {
                write!(formatter, "images/planets/arid_world_1.png")
            }
            PlanetSprites::AridWorld2 => {
                write!(formatter, "images/planets/arid_world_2.png")
            }
            PlanetSprites::AridWorld3 => {
                write!(formatter, "images/planets/arid_world_3.png")
            }
            PlanetSprites::AridWorld4 => {
                write!(formatter, "images/planets/arid_world_4.png")
            }
            PlanetSprites::AridWorld5 => {
                write!(formatter, "images/planets/arid_world_5.png")
            }
            PlanetSprites::AridWorld6 => {
                write!(formatter, "images/planets/arid_world_6.png")
            }
            PlanetSprites::AridWorld7 => {
                write!(formatter, "images/planets/arid_world_7.png")
            }
            PlanetSprites::AridWorld8 => {
                write!(formatter, "images/planets/arid_world_8.png")
            }
            PlanetSprites::AridWorld9 => {
                write!(formatter, "images/planets/arid_world_9.png")
            }
            PlanetSprites::AridWorld10 => {
                write!(formatter, "images/planets/arid_world_10.png")
            }
            PlanetSprites::BarrenWorld1 => {
                write!(formatter, "images/planets/barren_world_1.png")
            }
            PlanetSprites::BarrenWorld2 => {
                write!(formatter, "images/planets/barren_world_2.png")
            }
            PlanetSprites::BarrenWorld3 => {
                write!(formatter, "images/planets/barren_world_3.png")
            }
            PlanetSprites::BarrenWorld4 => {
                write!(formatter, "images/planets/barren_world_4.png")
            }
            PlanetSprites::BarrenWorld5 => {
                write!(formatter, "images/planets/barren_world_5.png")
            }
            PlanetSprites::BarrenWorld6 => {
                write!(formatter, "images/planets/barren_world_6.png")
            }
            PlanetSprites::BarrenWorld7 => {
                write!(formatter, "images/planets/barren_world_7.png")
            }
            PlanetSprites::BarrenWorld8 => {
                write!(formatter, "images/planets/barren_world_8.png")
            }
            PlanetSprites::BarrenWorld9 => {
                write!(formatter, "images/planets/barren_world_9.png")
            }
            PlanetSprites::BarrenWorld10 => {
                write!(formatter, "images/planets/barren_world_10.png")
            }
            PlanetSprites::GasGiantWorld1 => {
                write!(formatter, "images/planets/gas_giant_world_1.png")
            }
            PlanetSprites::GasGiantWorld2 => {
                write!(formatter, "images/planets/gas_giant_world_2.png")
            }
            PlanetSprites::GasGiantWorld3 => {
                write!(formatter, "images/planets/gas_giant_world_3.png")
            }
            PlanetSprites::GasGiantWorld4 => {
                write!(formatter, "images/planets/gas_giant_world_4.png")
            }
            PlanetSprites::GasGiantWorld5 => {
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
            PlanetSprites::IceWorld1 => {
                write!(formatter, "images/planets/ice_world_1.png")
            }
            PlanetSprites::IceWorld2 => {
                write!(formatter, "images/planets/ice_world_2.png")
            }
            PlanetSprites::IceWorld3 => {
                write!(formatter, "images/planets/ice_world_3.png")
            }
            PlanetSprites::IceWorld4 => {
                write!(formatter, "images/planets/ice_world_4.png")
            }
            PlanetSprites::IceWorld5 => {
                write!(formatter, "images/planets/ice_world_5.png")
            }
            PlanetSprites::IceWorld6 => {
                write!(formatter, "images/planets/ice_world_6.png")
            }
            PlanetSprites::IceWorld7 => {
                write!(formatter, "images/planets/ice_world_7.png")
            }
            PlanetSprites::IceWorld8 => {
                write!(formatter, "images/planets/ice_world_8.png")
            }
            PlanetSprites::IceWorld9 => {
                write!(formatter, "images/planets/ice_world_9.png")
            }
            PlanetSprites::IceWorld10 => {
                write!(formatter, "images/planets/ice_world_10.png")
            }
            PlanetSprites::ParadiseWorld1 => {
                write!(formatter, "images/planets/paradise_world_1.png")
            }
            PlanetSprites::ParadiseWorld2 => {
                write!(formatter, "images/planets/paradise_world_2.png")
            }
            PlanetSprites::ParadiseWorld3 => {
                write!(formatter, "images/planets/paradise_world_3.png")
            }
            PlanetSprites::ParadiseWorld4 => {
                write!(formatter, "images/planets/paradise_world_4.png")
            }
            PlanetSprites::ParadiseWorld5 => {
                write!(formatter, "images/planets/paradise_world_5.png")
            }
            PlanetSprites::ParadiseWorld6 => {
                write!(formatter, "images/planets/paradise_world_6.png")
            }
            PlanetSprites::ParadiseWorld7 => {
                write!(formatter, "images/planets/paradise_world_7.png")
            }
            PlanetSprites::ParadiseWorld8 => {
                write!(formatter, "images/planets/paradise_world_8.png")
            }
            PlanetSprites::ParadiseWorld9 => {
                write!(formatter, "images/planets/paradise_world_9.png")
            }
            PlanetSprites::ParadiseWorld10 => {
                write!(formatter, "images/planets/paradise_world_10.png")
            }
            PlanetSprites::WetWorld1 => {
                write!(formatter, "images/planets/wet_world_1.png")
            }
            PlanetSprites::WetWorld2 => {
                write!(formatter, "images/planets/wet_world_2.png")
            }
            PlanetSprites::WetWorld3 => {
                write!(formatter, "images/planets/wet_world_3.png")
            }
            PlanetSprites::WetWorld4 => {
                write!(formatter, "images/planets/wet_world_4.png")
            }
            PlanetSprites::WetWorld5 => {
                write!(formatter, "images/planets/wet_world_5.png")
            }
            PlanetSprites::WetWorld6 => {
                write!(formatter, "images/planets/wet_world_6.png")
            }
            PlanetSprites::WetWorld7 => {
                write!(formatter, "images/planets/wet_world_7.png")
            }
            PlanetSprites::WetWorld8 => {
                write!(formatter, "images/planets/wet_world_8.png")
            }
            PlanetSprites::WetWorld9 => {
                write!(formatter, "images/planets/wet_world_9.png")
            }
            PlanetSprites::WetWorld10 => {
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
