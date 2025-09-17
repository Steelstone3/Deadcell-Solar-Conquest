use bevy::ecs::query::QueryData;

use crate::components::user_interface::multiple_selection_box::MultipleSelectionBox;

use bevy::{
    prelude::{Entity, Transform},
    sprite::Sprite,
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectionBoxQuery {
    pub entity: Entity,
    pub multiple_selection_box: &'static mut MultipleSelectionBox,
}
