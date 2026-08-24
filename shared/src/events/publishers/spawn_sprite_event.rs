use crate::events::publishers::spawn_entity::SpawnEntity;
use bevy::ecs::message::Message;

#[derive(Message)]
pub struct SpawnSpriteEvent {
    pub spawn_sprite: SpawnEntity,
}

impl SpawnSpriteEvent {
    pub fn spawn_sprite(spawn_sprite: SpawnEntity) -> Self {
        Self { spawn_sprite }
    }
}
