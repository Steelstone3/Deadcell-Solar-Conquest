use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum KarcanIcon {
    Battlecruiser,
    Bomber,
    Dreadnought,
    Fighter,
    Frigate,
    Scout,
    SupportShip,
    TorpedoShip,
}

impl Display for KarcanIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KarcanIcon::Battlecruiser => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_battlecruiser_icon.png"
                )
            }
            KarcanIcon::Bomber => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_bomber_icon.png"
                )
            }
            KarcanIcon::Dreadnought => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_dreadnought_icon.png"
                )
            }
            KarcanIcon::Fighter => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_fighter_icon.png"
                )
            }
            KarcanIcon::Frigate => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_frigate_icon.png"
                )
            }
            KarcanIcon::Scout => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_scout_icon.png"
                )
            }
            KarcanIcon::SupportShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_support_ship_icon.png"
                )
            }
            KarcanIcon::TorpedoShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_torpedo_ship_icon.png"
                )
            }
        }
    }
}
