use crate::components::faction::starbase::Starbase;
use bevy::{ecs::query::QueryData, transform::components::Transform};

#[derive(QueryData)]
pub struct SpaceStationQuery {
    pub transform: &'static Transform,
    pub space_facility: &'static Starbase,
}
