use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
    transform::components::Transform,
    utils::default,
};

#[derive(PartialEq)]
pub enum SpriteType {
    Static,
    Animated,
}

#[derive(Event)]
pub struct SpawnSpriteEvent {
    pub sprite_type: SpriteType,
    pub spawn_sprite: SpawnSprite,
    pub spawn_animated_sprite: SpawnAnimatedSprite,
}

impl SpawnSpriteEvent {
    pub fn spawn_sprite(spawn_sprite: SpawnSprite) -> Self {
        Self {
            sprite_type: SpriteType::Static,
            spawn_sprite,
            spawn_animated_sprite: default(),
        }
    }

    pub fn spawn_animated_sprite(
        spawn_sprite: SpawnSprite,
        spawn_animated_sprite: SpawnAnimatedSprite,
    ) -> Self {
        Self {
            sprite_type: SpriteType::Animated,
            spawn_sprite,
            spawn_animated_sprite,
        }
    }
}

pub struct SpawnSprite {
    pub sprite_path: String,
    pub size: Vec2,
    pub transform: Transform,
    pub entity: Entity,
}

#[derive(Default)]
pub struct SpawnAnimatedSprite {
    pub sprite_tile_size: u32,
    pub frame_timing: f32,
    pub frame_count: usize,
}
