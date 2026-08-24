use crate::events::publishers::{
    spawn_animated_sprite::SpawnAnimatedSprite, spawn_entity::SpawnEntity,
};
use bevy::ecs::message::Message;

#[derive(Message)]
pub struct SpawnAnimatedSpriteEvent {
    pub spawn_sprite: SpawnEntity,
    pub spawn_animated_sprite: SpawnAnimatedSprite,
}

impl SpawnAnimatedSpriteEvent {
    pub fn spawn_animated_sprite(
        spawn_sprite: SpawnEntity,
        spawn_animated_sprite: SpawnAnimatedSprite,
    ) -> Self {
        Self {
            spawn_sprite,
            spawn_animated_sprite,
        }
    }
}
