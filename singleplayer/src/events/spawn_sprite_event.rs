use bevy::{
    ecs::{entity::Entity, message::Message},
    math::Vec2,
    transform::components::Transform,
};

#[derive(Message)]
pub struct SpawnSpriteEvent {
    pub spawn_sprite: SpawnSprite,
}

impl SpawnSpriteEvent {
    pub fn spawn_sprite(spawn_sprite: SpawnSprite) -> Self {
        Self { spawn_sprite }
    }
}

pub struct SpawnSprite {
    pub sprite_path: String,
    pub size: Vec2,
    pub transform: Transform,
    pub entity: Entity,
}
