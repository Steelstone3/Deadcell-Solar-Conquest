use bevy::prelude::Event;

use crate::components::user_interface::multiple_selection_box::SelectionArea;

#[derive(Event)]
pub struct SelectionAreaEvent {
    #[allow(dead_code)]
    pub selection_area: SelectionArea,
}
