use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum NoozlerIcon {
    Battlecruiser,
    Bomber,
    Dreadnought,
    Fighter,
    Frigate,
    Scout,
    SupportShip,
    TorpedoShip,
}

impl Display for NoozlerIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoozlerIcon::Battlecruiser => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_battlecruiser_icon.png"
                )
            }
            NoozlerIcon::Bomber => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_bomber_icon.png"
                )
            }
            NoozlerIcon::Dreadnought => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_dreadnought_icon.png"
                )
            }
            NoozlerIcon::Fighter => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_fighter_icon.png"
                )
            }
            NoozlerIcon::Frigate => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_frigate_icon.png"
                )
            }
            NoozlerIcon::Scout => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_scout_icon.png"
                )
            }
            NoozlerIcon::SupportShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_support_ship_icon.png"
                )
            }
            NoozlerIcon::TorpedoShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_torpedo_ship_icon.png"
                )
            }
        }
    }
}
