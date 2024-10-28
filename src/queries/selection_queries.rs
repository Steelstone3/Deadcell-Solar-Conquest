use bevy::ecs::query::QueryData;

use crate::components::user_interface::multiple_selection_box::MultipleSelectionBox;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectionBoxQuery {
    pub multiple_selection_box: &'static mut MultipleSelectionBox,
}
