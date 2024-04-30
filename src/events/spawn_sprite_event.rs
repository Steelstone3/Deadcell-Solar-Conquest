use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
    transform::components::Transform,
};

#[derive(Event)]
pub struct SpawnSpriteEvent {
    pub sprite_path: String,
    pub size: Vec2,
    pub transform: Transform,
    pub entity: Entity,
}
