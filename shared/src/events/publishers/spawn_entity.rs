use bevy::{
    ecs::entity::Entity,
    math::Vec2,
    transform::components::Transform,
};

pub struct SpawnEntity {
    pub sprite_path: String,
    pub size: Vec2,
    pub transform: Transform,
    pub entity: Entity,
}
