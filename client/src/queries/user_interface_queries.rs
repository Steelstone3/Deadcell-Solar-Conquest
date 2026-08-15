use crate::components::{
    faction::{starbase::Starbase, starship::Starship},
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
    pub starbase: Option<&'static Starbase>,
    pub starship: Option<&'static Starship>,
}
