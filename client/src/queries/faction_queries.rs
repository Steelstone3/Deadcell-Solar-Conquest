use bevy::{ecs::query::QueryData, transform::components::Transform};
use deadcell_solar_conquest_shared::components::client::faction::starbase::Starbase;

#[derive(QueryData)]
pub struct SpaceStationQuery {
    pub transform: &'static Transform,
    pub space_facility: &'static Starbase,
}
