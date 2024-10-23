use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum StarshipIcon {
    AtarkSupportShip,
    AtarkScout,
    AtarkFighter,
    AtarkTorpedoShip,
    AtarkBomber,
    AtarkFrigate,
    AtarkBattleCruiser,
    AtarkDreadnought,
    KarcanSupportShip,
    KarcanScout,
    KarcanFighter,
    KarcanTorpedoShip,
    KarcanBomber,
    KarcanFrigate,
    KarcanBattleCruiser,
    KarcanDreadnought,
    NoozlerSupportShip,
    NoozlerScout,
    NoozlerFighter,
    NoozlerTorpedoShip,
    NoozlerBomber,
    NoozlerFrigate,
    NoozlerBattleCruiser,
    NoozlerDreadnought,
    GarnokSupportShip,
    GarnokScout,
    GarnokFighter,
    GarnokTorpedoShip,
    GarnokBomber,
    GarnokFrigate,
    GarnokBattleCruiser,
    GarnokDreadnought,
    None,
}

impl Display for StarshipIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipIcon::AtarkSupportShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_support_ship_icon.png"
                )
            }
            StarshipIcon::AtarkScout => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_scout_icon.png"
                )
            }
            StarshipIcon::AtarkFighter => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_fighter_icon.png"
                )
            }
            StarshipIcon::AtarkTorpedoShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_torpedo_ship_icon.png"
                )
            }
            StarshipIcon::AtarkBomber => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_bomber_icon.png"
                )
            }
            StarshipIcon::AtarkFrigate => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_frigate_icon.png"
                )
            }
            StarshipIcon::AtarkBattleCruiser => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_battlecruiser_icon.png"
                )
            }
            StarshipIcon::AtarkDreadnought => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_dreadnought_icon.png"
                )
            }
            StarshipIcon::KarcanSupportShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_support_ship_icon.png"
                )
            }
            StarshipIcon::KarcanScout => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_scout_icon.png"
                )
            }
            StarshipIcon::KarcanFighter => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_fighter_icon.png"
                )
            }
            StarshipIcon::KarcanTorpedoShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_torpedo_ship_icon.png"
                )
            }
            StarshipIcon::KarcanBomber => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_bomber_icon.png"
                )
            }
            StarshipIcon::KarcanFrigate => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_frigate_icon.png"
                )
            }
            StarshipIcon::KarcanBattleCruiser => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_battlecruiser_icon.png"
                )
            }
            StarshipIcon::KarcanDreadnought => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_dreadnought_icon.png"
                )
            }
            StarshipIcon::NoozlerSupportShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_support_ship_icon.png"
                )
            }
            StarshipIcon::NoozlerScout => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_scout_icon.png"
                )
            }
            StarshipIcon::NoozlerFighter => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_fighter_icon.png"
                )
            }
            StarshipIcon::NoozlerTorpedoShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_torpedo_ship_icon.png"
                )
            }
            StarshipIcon::NoozlerBomber => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_bomber_icon.png"
                )
            }
            StarshipIcon::NoozlerFrigate => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_frigate_icon.png"
                )
            }
            StarshipIcon::NoozlerBattleCruiser => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_battlecruiser_icon.png"
                )
            }
            StarshipIcon::NoozlerDreadnought => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokSupportShip =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokScout =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokFighter =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokTorpedoShip =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokBomber =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokFrigate =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokBattleCruiser =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::GarnokDreadnought =>  {
                write!(
                    formatter,
                    "user_interface/icons/starships/garnok/garnok_dreadnought_icon.png"
                )
            }
            StarshipIcon::None => write!(formatter, ""),
        }
    }
}
