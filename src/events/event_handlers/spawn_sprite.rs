use crate::events::spawn_sprite_event::SpawnSpriteEvent;
use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    sprite::{Sprite, SpriteBundle},
};

pub fn spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_sprite_events: EventReader<SpawnSpriteEvent>,
) {
    for spawn_sprite_event in spawn_sprite_events.read() {
        if let Some(mut entity) = commands.get_entity(spawn_sprite_event.entity) {
            let texture = asset_server.load(&spawn_sprite_event.sprite_path);

            entity.insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(spawn_sprite_event.size),
                    ..Default::default()
                },
                texture,
                transform: spawn_sprite_event.transform,
                ..Default::default()
            });
        }
    }
}
