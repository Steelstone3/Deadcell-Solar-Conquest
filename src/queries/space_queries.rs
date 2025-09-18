use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::map::star::Star;

#[derive(QueryData)]
pub struct StarQuery {
    pub transform: &'static Transform,
    pub star: &'static Star,
}
