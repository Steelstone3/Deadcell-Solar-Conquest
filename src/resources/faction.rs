use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use crate::assets::images::starship_sprite::StarshipSprite;

#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerFaction {
    pub player_faction: Faction,
}

#[derive(Default, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Faction {
    GranokImperialEmpire,
    StarGuardAlliance,
    #[default]
    UniversalMechanicalContigent,
    VoidwalkerCollective,
    None
}

impl Faction {
    pub fn determine_faction(starship_sprite: StarshipSprite) -> Faction {
        match starship_sprite {
            StarshipSprite::GranokImperialEmpireCorvette => Faction::GranokImperialEmpire,
            StarshipSprite::GranokImperialEmpireDestroyer => Faction::GranokImperialEmpire,
            StarshipSprite::GranokImperialEmpireFighter => Faction::GranokImperialEmpire,
            StarshipSprite::StarGuardAllianceBattleCruiser => Faction::StarGuardAlliance,
            StarshipSprite::StarGuardAllianceBattleship => Faction::StarGuardAlliance,
            StarshipSprite::StarGuardAllianceCorvette => Faction::StarGuardAlliance,
            StarshipSprite::StarGuardAllianceDestroyer => Faction::StarGuardAlliance,
            StarshipSprite::StarGuardAllianceTorpedoShip => Faction::StarGuardAlliance,
            StarshipSprite::UniversalMechanicalContingentDestroyer => Faction::UniversalMechanicalContigent,
            StarshipSprite::UniversalMechanicalContingentIntelShip => Faction::UniversalMechanicalContigent,
            StarshipSprite::VoidwalkerCollectiveDreadnought => Faction::VoidwalkerCollective,
            StarshipSprite::VoidwalkerCollectiveFighter => Faction::VoidwalkerCollective,
            StarshipSprite::None => Faction::None,
        }
    }
}
