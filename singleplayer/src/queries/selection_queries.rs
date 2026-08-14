use crate::components::user_interface::multiple_selection_box::MultipleSelectionBox;
use bevy::{ecs::query::QueryData, prelude::Entity};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectionBoxQuery {
    pub entity: Entity,
    pub multiple_selection_box: &'static mut MultipleSelectionBox,
}
