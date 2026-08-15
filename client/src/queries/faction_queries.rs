use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::faction::starbase::Starbase;

#[derive(QueryData)]
pub struct SpaceStationQuery {
    pub transform: &'static Transform,
    pub space_facility: &'static Starbase,
}
