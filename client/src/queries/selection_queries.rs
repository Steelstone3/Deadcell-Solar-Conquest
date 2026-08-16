use bevy::{ecs::query::QueryData, prelude::Entity};
use deadcell_solar_conquest_shared::components::client::user_interface::multiple_selection_box::MultipleSelectionBox;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectionBoxQuery {
    pub entity: Entity,
    pub multiple_selection_box: &'static mut MultipleSelectionBox,
}
