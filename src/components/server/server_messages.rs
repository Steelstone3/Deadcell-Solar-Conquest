use bevy::ecs::component::Component;
use bevy_renet::renet;
use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}
