use bevy::{ecs::query::QueryData, transform::components::Transform};
use deadcell_solar_conquest_shared::components::client::map::star::Star;

#[derive(QueryData)]
pub struct StarQuery {
    pub transform: &'static Transform,
    pub star: &'static Star,
}
