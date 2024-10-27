use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::map::sun::Sun;

#[derive(QueryData)]
pub struct SunQuery {
    pub transform: &'static Transform,
    pub sun: &'static Sun,
}
