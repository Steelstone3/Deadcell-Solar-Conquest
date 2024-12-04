use bevy::prelude::Event;
use bevy_renet::renet::ClientId;

#[derive(Event)]
pub struct PlayerConnectedEvent {
    #[allow(dead_code)]
    pub client_id: ClientId,
}
