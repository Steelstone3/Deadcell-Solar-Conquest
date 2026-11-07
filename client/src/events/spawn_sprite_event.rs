use bevy::{
    asset::AssetServer,
    ecs::{
        message::MessageReader,
        system::{Commands, Res},
    },
    sprite::Sprite,
};
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

pub fn spawn_sprite_event_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_sprite_events: MessageReader<SpawnSpriteEvent>,
) {
    for spawn_sprite_event in spawn_sprite_events.read() {
        if let Ok(mut entity) = commands.get_entity(spawn_sprite_event.spawn_sprite.entity) {
            let texture = asset_server.load(&spawn_sprite_event.spawn_sprite.sprite_path);

            let mut sprite = Sprite::from_image(texture);
            sprite.custom_size = Some(spawn_sprite_event.spawn_sprite.size);

            entity.insert((sprite, spawn_sprite_event.spawn_sprite.transform));
        }
    }
}
