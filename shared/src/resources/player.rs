use crate::resources::factions::PlayerFaction;
use bevy::ecs::resource::Resource;
use rand::random;

#[derive(Resource)]
pub struct Player {
    pub client_id: u64,
    pub faction: PlayerFaction,
    pub is_connected: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            client_id: random(),
            faction: Default::default(),
            is_connected: Default::default(),
        }
    }
}
