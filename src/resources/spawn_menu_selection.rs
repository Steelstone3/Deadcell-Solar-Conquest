use bevy::{
    ecs::system::{ResMut, Resource},
    prelude::Entity,
};

use crate::{
    assets::user_interface::icons::{
        space_facility_icons::SpaceFacilityIcon, starship_icons::StarshipIcon,
    },
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

pub const MAXIMUM_SELECTIONS: usize = 10;

#[derive(Resource)]
pub struct SpawnMenuSelection {
    pub selection: SpawnSelection,
    pub selected_entities: [Entity; MAXIMUM_SELECTIONS],
    pub starship_selection: StarshipIcon,
    pub space_facility_selection: SpaceFacilityIcon,
}

impl Default for SpawnMenuSelection {
    fn default() -> Self {
        Self {
            selection: SpawnSelection::None,
            selected_entities: [
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
                Entity::PLACEHOLDER,
            ],
            starship_selection: StarshipIcon::None,
            space_facility_selection: SpaceFacilityIcon::None,
        }
    }
}

impl SpawnMenuSelection {
    pub fn reset_all(spawn_menu_selection: &mut ResMut<'_, SpawnMenuSelection>) {
        spawn_menu_selection.selection = SpawnSelection::None;
        spawn_menu_selection.selected_entities = [
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
            Entity::PLACEHOLDER,
        ];
        spawn_menu_selection.starship_selection = StarshipIcon::None;
        spawn_menu_selection.space_facility_selection = SpaceFacilityIcon::None;
    }

    pub fn default_selection(spawn_menu_selection: &mut ResMut<'_, SpawnMenuSelection>) {
        spawn_menu_selection.selection = SpawnSelection::None;
        spawn_menu_selection.starship_selection = StarshipIcon::None;
        spawn_menu_selection.space_facility_selection = SpaceFacilityIcon::None;
    }

    pub fn add_selection(&mut self, entity: Entity) {
        let mut index = 0;

        for selected_entity in self.selected_entities {
            if selected_entity == Entity::PLACEHOLDER {
                break;
            }

            index += 1;
        }

        if index <= self.selected_entities.len() - 1 && !self.selected_entities.contains(&entity) {
            self.selected_entities[index as usize] = entity;
        }
    }
}
