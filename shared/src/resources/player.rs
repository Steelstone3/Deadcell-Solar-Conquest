use crate::resources::factions::PlayerFaction;
use bevy::ecs::resource::Resource;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Debug, Resource, Serialize, Deserialize, Clone)]
pub struct Player {
    pub client_id: u64,
    pub faction: PlayerFaction,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            client_id: random(),
            faction: Default::default(),
        }
    }
}
