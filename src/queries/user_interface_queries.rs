use crate::components::{
    space_facility::SpaceFacility,
    space_station::SpaceStation,
    starship::Starship,
    user_interface::{Selectable, SelectedSprite},
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
    pub space_station: Option<&'static SpaceStation>,
    pub space_facility: Option<&'static SpaceFacility>,
    pub spaceship: Option<&'static Starship>,
}
