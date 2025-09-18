use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::map::star::Star;

#[derive(QueryData)]
pub struct SunQuery {
    pub transform: &'static Transform,
    pub sun: &'static Star,
}
