use bevy::ecs::event::Event;

use super::spawn_sprite_event::SpawnSpriteEvent;

#[derive(Event)]
pub struct SpawnAnimatedSpriteEvent {
    pub sprite_tile_size: f32,
    pub frame_timing: f32,
    pub frame_count: usize,
    pub spawn_sprite_event: SpawnSpriteEvent,
}
