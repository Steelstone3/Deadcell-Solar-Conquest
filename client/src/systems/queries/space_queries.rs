use bevy::{ecs::query::QueryData, transform::components::Transform};
use deadcell_solar_conquest_shared::components::space::Space;

#[derive(QueryData)]
pub struct SpaceTransformQuery {
    pub transform: &'static Transform,
    pub space: &'static Space,
}
