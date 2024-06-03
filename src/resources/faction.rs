use std::fmt::Display;

use bevy::prelude::Resource;

use crate::assets::{
    images::{
        faction_starship_sprite::starship_sprite::StarshipSprite,
        space_facility_sprite::SpaceStationSprite,
    },
    user_interace::icons::{space_facility_icons::SpaceFacilityIcon, starship_icons::StarshipIcon},
};

#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerFaction {
    pub player_faction: Faction,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Faction {
    Atark,
    #[default]
    Karcan,
    Noozler,
}

impl Faction {
    pub fn determine_faction(starship_sprite: StarshipSprite) -> Faction {
        match starship_sprite {
            StarshipSprite::AtarkBattleCruiser => Faction::Atark,
            StarshipSprite::AtarkBomber => Faction::Atark,
            StarshipSprite::AtarkDreadnought => Faction::Atark,
            StarshipSprite::AtarkFighter => Faction::Atark,
            StarshipSprite::AtarkFrigate => Faction::Atark,
            StarshipSprite::AtarkScout => Faction::Atark,
            StarshipSprite::AtarkSupportShip => Faction::Atark,
            StarshipSprite::AtarkTorpedoShip => Faction::Atark,
            StarshipSprite::KarcanBattleCruiser => Faction::Karcan,
            StarshipSprite::KarcanBomber => Faction::Karcan,
            StarshipSprite::KarcanDreadnought => Faction::Karcan,
            StarshipSprite::KarcanFighter => Faction::Karcan,
            StarshipSprite::KarcanFrigate => Faction::Karcan,
            StarshipSprite::KarcanScout => Faction::Karcan,
            StarshipSprite::KarcanSupportShip => Faction::Karcan,
            StarshipSprite::KarcanTorpedoShip => Faction::Karcan,
            StarshipSprite::NoozlerBattleCruiser => Faction::Noozler,
            StarshipSprite::NoozlerBomber => Faction::Noozler,
            StarshipSprite::NoozlerDreadnought => Faction::Noozler,
            StarshipSprite::NoozlerFighter => Faction::Noozler,
            StarshipSprite::NoozlerFrigate => Faction::Noozler,
            StarshipSprite::NoozlerScout => Faction::Noozler,
            StarshipSprite::NoozlerSupportShip => Faction::Noozler,
            StarshipSprite::NoozlerTorpedoShip => Faction::Noozler,
        }
    }
}

#[derive(PartialEq)]
pub enum StarshipType {
    SupportShip,
    Scout,
    Fighter,
    TorpedoShip,
    Bomber,
    Frigate,
    BattleCruiser,
    Dreadnought,
}

impl Display for StarshipType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipType::SupportShip => write!(formatter, "Support"),
            StarshipType::Scout => write!(formatter, "Scout"),
            StarshipType::Fighter => write!(formatter, "Fighter"),
            StarshipType::TorpedoShip => write!(formatter, "Torpedo"),
            StarshipType::Bomber => write!(formatter, "Bomber"),
            StarshipType::Frigate => write!(formatter, "Frigate"),
            StarshipType::BattleCruiser => write!(formatter, "BattleCruiser"),
            StarshipType::Dreadnought => write!(formatter, "Dreadnought"),
        }
    }
}

impl StarshipType {
    pub fn icon_convert_from(&self, faction: Faction) -> StarshipIcon {
        match faction {
            Faction::Atark => match self {
                StarshipType::SupportShip => StarshipIcon::AtarkSupportShip,
                StarshipType::Scout => StarshipIcon::AtarkScout,
                StarshipType::Fighter => StarshipIcon::AtarkFighter,
                StarshipType::TorpedoShip => StarshipIcon::AtarkTorpedoShip,
                StarshipType::Bomber => StarshipIcon::AtarkBomber,
                StarshipType::Frigate => StarshipIcon::AtarkFrigate,
                StarshipType::BattleCruiser => StarshipIcon::AtarkBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::AtarkDreadnought,
            },
            Faction::Karcan => match self {
                StarshipType::SupportShip => StarshipIcon::KarcanSupportShip,
                StarshipType::Scout => StarshipIcon::KarcanScout,
                StarshipType::Fighter => StarshipIcon::KarcanFighter,
                StarshipType::TorpedoShip => StarshipIcon::KarcanTorpedoShip,
                StarshipType::Bomber => StarshipIcon::KarcanBomber,
                StarshipType::Frigate => StarshipIcon::KarcanFrigate,
                StarshipType::BattleCruiser => StarshipIcon::KarcanBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::KarcanDreadnought,
            },
            Faction::Noozler => match self {
                StarshipType::SupportShip => StarshipIcon::NoozlerSupportShip,
                StarshipType::Scout => StarshipIcon::NoozlerScout,
                StarshipType::Fighter => StarshipIcon::NoozlerFighter,
                StarshipType::TorpedoShip => StarshipIcon::NoozlerTorpedoShip,
                StarshipType::Bomber => StarshipIcon::NoozlerBomber,
                StarshipType::Frigate => StarshipIcon::NoozlerFrigate,
                StarshipType::BattleCruiser => StarshipIcon::NoozlerBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::NoozlerDreadnought,
            },
        }
    }

    pub fn sprite_convert_from(&self, faction: Faction) -> StarshipSprite {
        match faction {
            Faction::Atark => match self {
                StarshipType::SupportShip => StarshipSprite::AtarkSupportShip,
                StarshipType::Scout => StarshipSprite::AtarkScout,
                StarshipType::Fighter => StarshipSprite::AtarkFighter,
                StarshipType::TorpedoShip => StarshipSprite::AtarkTorpedoShip,
                StarshipType::Bomber => StarshipSprite::AtarkBomber,
                StarshipType::Frigate => StarshipSprite::AtarkFrigate,
                StarshipType::BattleCruiser => StarshipSprite::AtarkBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::AtarkDreadnought,
            },
            Faction::Karcan => match self {
                StarshipType::SupportShip => StarshipSprite::KarcanSupportShip,
                StarshipType::Scout => StarshipSprite::KarcanScout,
                StarshipType::Fighter => StarshipSprite::KarcanFighter,
                StarshipType::TorpedoShip => StarshipSprite::KarcanTorpedoShip,
                StarshipType::Bomber => StarshipSprite::KarcanBomber,
                StarshipType::Frigate => StarshipSprite::KarcanFrigate,
                StarshipType::BattleCruiser => StarshipSprite::KarcanBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::KarcanDreadnought,
            },
            Faction::Noozler => match self {
                StarshipType::SupportShip => StarshipSprite::NoozlerSupportShip,
                StarshipType::Scout => StarshipSprite::NoozlerScout,
                StarshipType::Fighter => StarshipSprite::NoozlerFighter,
                StarshipType::TorpedoShip => StarshipSprite::NoozlerTorpedoShip,
                StarshipType::Bomber => StarshipSprite::NoozlerBomber,
                StarshipType::Frigate => StarshipSprite::NoozlerFrigate,
                StarshipType::BattleCruiser => StarshipSprite::NoozlerBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::NoozlerDreadnought,
            },
        }
    }
}

pub enum StarStationType {
    SpaceStation,
}

impl StarStationType {
    pub fn sprite_convert_from(&self, faction: Faction) -> SpaceStationSprite {
        match faction {
            Faction::Atark => match self {
                StarStationType::SpaceStation => SpaceStationSprite::AtarkSpaceStation,
            },
            Faction::Karcan => match self {
                StarStationType::SpaceStation => SpaceStationSprite::KarcanSpaceStation,
            },
            Faction::Noozler => match self {
                StarStationType::SpaceStation => SpaceStationSprite::NoozlerSpaceStation,
            },
        }
    }
}

pub enum SpaceFacilityType {
    SpaceShipConstructionYard,
}

impl SpaceFacilityType {
    pub fn icon_convert_from(&self, faction: Faction) -> SpaceFacilityIcon {
        match faction {
            Faction::Atark => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::AtarkSpaceShipConstructionYard
                }
            },
            Faction::Karcan => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::KarcanSpaceShipConstructionYard
                }
            },
            Faction::Noozler => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::NoozlerSpaceShipConstructionYard
                }
            },
        }
    }
}
