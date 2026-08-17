use crate::components::map::star::Star;
use bevy::{ecs::query::QueryData, transform::components::Transform};

#[derive(QueryData)]
pub struct StarQuery {
    pub transform: &'static Transform,
    pub star: &'static Star,
}
