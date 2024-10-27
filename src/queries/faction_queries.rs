use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::faction::space_facility::SpaceFacility;

#[derive(QueryData)]
pub struct SpaceStationQuery {
    pub transform: &'static Transform,
    pub space_facility: &'static SpaceFacility,
}
