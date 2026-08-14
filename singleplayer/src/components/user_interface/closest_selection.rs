use bevy::{
    prelude::{Entity, Transform},
    sprite::Sprite,
};

pub struct ClosestSelection {
    pub transform: Transform,
    pub sprite: Sprite,
    pub distance: f32,
    pub entity: Entity,
}

impl Default for ClosestSelection {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            sprite: Sprite::default(),
            distance: -1.0,
            entity: Entity::PLACEHOLDER,
        }
    }
}

impl ClosestSelection {
    pub fn new(transform: Transform, sprite: Sprite, distance: f32, entity: Entity) -> Self {
        Self {
            transform,
            sprite,
            distance,
            entity,
        }
    }
}
