use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Tracking {
    pub entity_to_follow: Entity,
}
