use bevy::ecs::{entity::Entity, resource::Resource};
use bevy_renet::renet::ClientId;
use std::collections::HashMap;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}
