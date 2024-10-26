use crate::components::sun::Sun;
use bevy::{ecs::query::QueryData, transform::components::Transform};

#[derive(QueryData)]
pub struct SunQuery {
    pub transform: &'static Transform,
    pub sun: &'static Sun,
}
