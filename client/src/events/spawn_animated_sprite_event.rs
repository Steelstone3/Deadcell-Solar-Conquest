use bevy::ecs::event::Event;

use crate::events::spawn_sprite_event::SpawnSprite;

#[derive(Event)]
pub struct SpawnAnimatedSpriteEvent {
    pub spawn_sprite: SpawnSprite,
    pub spawn_animated_sprite: SpawnAnimatedSprite,
}

impl SpawnAnimatedSpriteEvent {
    pub fn spawn_animated_sprite(
        spawn_sprite: SpawnSprite,
        spawn_animated_sprite: SpawnAnimatedSprite,
    ) -> Self {
        Self {
            spawn_sprite,
            spawn_animated_sprite,
        }
    }
}

pub struct SpawnAnimatedSprite {
    pub sprite_tile_size: u32,
    pub frame_timing: f32,
    pub frame_count: usize,
}
