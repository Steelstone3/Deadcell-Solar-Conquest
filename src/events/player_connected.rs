use bevy::prelude::Event;
use bevy_renet::renet::ClientId;

#[derive(Event)]
pub struct PlayerConnectedEvent {
    pub client_id: ClientId,
}
