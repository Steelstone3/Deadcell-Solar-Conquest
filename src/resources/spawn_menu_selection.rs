use bevy::{ecs::system::Resource, prelude::Entity};

use crate::{
    assets::user_interface::icons::{
        space_facility_icons::SpaceFacilityIcon, starship_icons::StarshipIcon,
    },
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

#[derive(Resource)]
pub struct SpawnMenuSelection {
    pub selection: SpawnSelection,
    pub selected_entity: Entity,
    pub selected_entities: Vec<Entity>,
    pub starship_selection: StarshipIcon,
    pub space_facility_selection: SpaceFacilityIcon,
}

impl Default for SpawnMenuSelection {
    fn default() -> Self {
        Self {
            selection: SpawnSelection::None,
            selected_entity: Entity::PLACEHOLDER,
            selected_entities: vec![],
            starship_selection: StarshipIcon::None,
            space_facility_selection: SpaceFacilityIcon::None,
        }
    }
}

impl SpawnMenuSelection {
    pub fn reset_all(&mut self) {
        self.selection = SpawnSelection::None;
        self.selected_entity = Entity::PLACEHOLDER;
        self.selected_entities = vec![];
        self.starship_selection = StarshipIcon::None;
        self.space_facility_selection = SpaceFacilityIcon::None;
    }

    pub fn default_selection(&mut self) {
        self.selection = SpawnSelection::None;
        self.starship_selection = StarshipIcon::None;
        self.space_facility_selection = SpaceFacilityIcon::None;
    }

    pub fn single_selection(&mut self, entity: Entity) {
        // reset selected entities
        self.selected_entities = vec![];

        self.selected_entity = entity;
    }

    pub fn multi_selection(&mut self, entity: Entity) {
        // reset selected entity
        self.selected_entity = Entity::PLACEHOLDER;

        if !self.selected_entities.contains(&entity) {
            self.selected_entities.push(entity);
        }
    }
}
