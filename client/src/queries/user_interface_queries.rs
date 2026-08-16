use bevy::{
    ecs::query::QueryData,
    prelude::{Entity, Transform},
    sprite::Sprite,
};
use deadcell_solar_conquest_shared::components::client::{faction::{starbase::Starbase, starship::Starship}, user_interface::selection::{Selectable, SelectedSprite}};

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
