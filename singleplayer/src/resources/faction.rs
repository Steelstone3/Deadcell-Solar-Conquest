use crate::assets::sprites::starship_sprites::StarshipSprites;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerFaction {
    pub player_faction: Faction,
}

#[derive(Default, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Faction {
    GranokImperialEmpire,
    #[default]
    StarGuardAlliance,
    UniversalMechanicalContigent,
    VoidwalkerCollective,
    None,
}

impl Faction {
    pub fn determine_faction(starship_sprite: StarshipSprites) -> Faction {
        match starship_sprite {
            StarshipSprites::GranokImperialEmpireCorvette => Faction::GranokImperialEmpire,
            StarshipSprites::GranokImperialEmpireDestroyer => Faction::GranokImperialEmpire,
            StarshipSprites::GranokImperialEmpireFighter => Faction::GranokImperialEmpire,
            StarshipSprites::StarGuardAllianceBattleCruiser => Faction::StarGuardAlliance,
            StarshipSprites::StarGuardAllianceBattleship => Faction::StarGuardAlliance,
            StarshipSprites::StarGuardAllianceCorvette => Faction::StarGuardAlliance,
            StarshipSprites::StarGuardAllianceDestroyer => Faction::StarGuardAlliance,
            StarshipSprites::StarGuardAllianceTorpedoShip => Faction::StarGuardAlliance,
            StarshipSprites::UniversalMechanicalContingentDestroyer => {
                Faction::UniversalMechanicalContigent
            }
            StarshipSprites::UniversalMechanicalContingentIntelShip => {
                Faction::UniversalMechanicalContigent
            }
            StarshipSprites::VoidwalkerCollectiveDreadnought => Faction::VoidwalkerCollective,
            StarshipSprites::VoidwalkerCollectiveFighter => Faction::VoidwalkerCollective,
            StarshipSprites::None => Faction::None,
        }
    }
}
