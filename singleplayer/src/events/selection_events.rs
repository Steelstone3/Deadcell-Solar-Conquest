use crate::components::user_interface::multiple_selection_box::SelectionArea;
use bevy::ecs::message::Message;

#[derive(Message)]
pub struct SelectionAreaEvent {
    #[allow(dead_code)]
    pub selection_area: SelectionArea,
}
