use bevy::ecs::resource::Resource;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug)]
pub struct PlayerFaction {
    pub faction: Factions,
}

// TODO see if this pattern can be used to remove rand_derive2 dependency
impl Default for PlayerFaction {
    fn default() -> Self {
        let faction = match random::<u8>() % 4 {
            0 => Factions::GranokImperialEmpire,
            1 => Factions::StarGuardAlliance,
            2 => Factions::UniversalMechanicalContingent,
            _ => Factions::VoidwalkerCollective,
        };

        Self { faction }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Factions {
    GranokImperialEmpire,
    StarGuardAlliance,
    UniversalMechanicalContingent,
    VoidwalkerCollective,
    None,
}
