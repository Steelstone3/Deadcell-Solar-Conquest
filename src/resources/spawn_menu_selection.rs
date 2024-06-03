use bevy::{
    ecs::system::{ResMut, Resource},
    prelude::Entity,
};

use crate::{
    assets::user_interace::icons::{
        space_facility_icons::SpaceFacilityIcon, starship_icons::StarshipIcon,
    },
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

#[derive(Resource)]
pub struct SpawnMenuSelection {
    pub selection: SpawnSelection,
    pub selected_entity: Entity,
    pub starship_selection: StarshipIcon,
    pub space_facility_selection: SpaceFacilityIcon,
}

impl Default for SpawnMenuSelection {
    fn default() -> Self {
        Self {
            selection: SpawnSelection::None,
            selected_entity: Entity::PLACEHOLDER,
            starship_selection: StarshipIcon::None,
            space_facility_selection: SpaceFacilityIcon::None,
        }
    }
}

impl SpawnMenuSelection {
    pub fn reset(selected_item: &mut ResMut<'_, SpawnMenuSelection>) {
        selected_item.selection = SpawnSelection::None;
        selected_item.selected_entity = Entity::PLACEHOLDER;
        selected_item.starship_selection = StarshipIcon::None;
        selected_item.space_facility_selection = SpaceFacilityIcon::None;
    }
}
