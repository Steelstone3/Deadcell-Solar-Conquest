use crate::assets::user_interface::selection_indicator::SelectionIndicator;
use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Selectable;

#[derive(Component, Clone, Copy)]
pub struct SelectedSprite {
    pub sprite_path: SelectionIndicator,
}

impl SelectedSprite {
    pub fn new(team_selection_sprite: SelectionIndicator) -> Self {
        Self {
            sprite_path: team_selection_sprite,
        }
    }
}
