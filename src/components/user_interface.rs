use crate::assets::user_interace::{
    icons::{space_facility_icons::SpaceFacilityIcon, starship_icons::StarshipIcon},
    team::TeamSelectionSprite,
};
use bevy::prelude::Component;

#[derive(Component)]
pub struct SpawnMenu;

#[derive(Component)]
pub struct SpawnSubMenuButton;

#[derive(Component)]
pub struct SpawnMenuButton;

#[derive(Component)]
pub struct SelectStarshipSpawnButton {
    pub icon: StarshipIcon,
}

#[derive(Component)]
pub struct SelectFacilitySpawnButton {
    pub icon: SpaceFacilityIcon,
}

// In Game Selection

#[derive(Component, Clone, Copy)]
pub struct Selectable;

#[derive(Component, Clone, Copy)]
pub struct SelectedSprite {
    pub sprite_path: TeamSelectionSprite,
}

impl SelectedSprite {
    pub fn new(sprite_path: TeamSelectionSprite) -> Self {
        Self { sprite_path }
    }
}
