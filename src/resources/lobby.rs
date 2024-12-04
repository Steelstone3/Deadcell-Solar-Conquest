use bevy::prelude::Resource;
use bevy_renet::renet::ClientId;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: Vec<ClientId>,
}
