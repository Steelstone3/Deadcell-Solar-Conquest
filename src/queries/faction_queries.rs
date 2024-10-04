use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::space_station::SpaceStation;

#[derive(QueryData)]
pub struct SpaceStationQuery {
    pub transform: &'static Transform,
    pub space_station: &'static SpaceStation,
}