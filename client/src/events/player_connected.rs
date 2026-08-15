use bevy::{ecs::message::Message, prelude::Event};
use bevy_renet::renet::ClientId;

#[derive(Message)]
pub struct PlayerConnectedEvent {
    #[allow(dead_code)]
    pub client_id: ClientId,
}
