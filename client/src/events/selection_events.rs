use bevy::ecs::message::Message;

use crate::components::user_interface::multiple_selection_box::SelectionArea;

#[derive(Message)]
pub struct SelectionAreaEvent {
    #[allow(dead_code)]
    pub selection_area: SelectionArea,
}
