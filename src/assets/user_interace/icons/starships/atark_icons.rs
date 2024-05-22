use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum AtarkIcon {
    Battlecruiser,
    Bomber,
    Dreadnought,
    Fighter,
    Frigate,
    Scout,
    SupportShip,
    TorpedoShip,
}

impl Display for AtarkIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtarkIcon::Battlecruiser => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_battlecruiser_icon.png"
                )
            }
            AtarkIcon::Bomber => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_bomber_icon.png"
                )
            }
            AtarkIcon::Dreadnought => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_dreadnought_icon.png"
                )
            }
            AtarkIcon::Fighter => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_fighter_icon.png"
                )
            }
            AtarkIcon::Frigate => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_frigate_icon.png"
                )
            }
            AtarkIcon::Scout => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_scout_icon.png"
                )
            }
            AtarkIcon::SupportShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_support_ship_icon.png"
                )
            }
            AtarkIcon::TorpedoShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_torpedo_ship_icon.png"
                )
            }
        }
    }
}
