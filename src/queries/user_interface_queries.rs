use crate::components::{
    faction::{space_facility::SpaceFacility, starship::Starship},
    user_interface::selection::{Selectable, SelectedSprite},
};
use bevy::{
    ecs::query::QueryData,
    prelude::{Entity, Transform},
    sprite::Sprite,
};

#[derive(QueryData)]
pub struct SelectableQuery {
    pub transform: &'static Transform,
    pub sprite: &'static Sprite,
    pub entity: Entity,
    pub selectable: &'static Selectable,
}

#[derive(QueryData)]
pub struct SelectionQuery {
    pub entity: Option<Entity>,
    pub selected: &'static SelectedSprite,
}

#[derive(QueryData)]
pub struct TypeCheckQuery {
    pub space_facility: Option<&'static SpaceFacility>,
    pub starship: Option<&'static Starship>,
}
