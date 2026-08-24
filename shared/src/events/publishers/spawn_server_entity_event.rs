use bevy::ecs::message::Message;
use crate::events::publishers::spawn_entity::SpawnEntity;

#[derive(Message)]
pub struct SpawnServerEntityEvent {
    pub spawn_server_entity: SpawnEntity,
}

impl SpawnServerEntityEvent {
    pub fn spawn_sprite(spawn_server_entity: SpawnEntity) -> Self {
        Self {
            spawn_server_entity: spawn_server_entity,
        }
    }
}
