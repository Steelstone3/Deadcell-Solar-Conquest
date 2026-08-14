use crate::assets::user_interface::selection_indicator::SelectionIndicator;
use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Selectable;

#[derive(Component, Clone, Copy)]
pub struct SelectedSprite {
    pub sprite_path: SelectionIndicator,
}

impl Default for SelectedSprite {
    fn default() -> Self {
        Self {
            sprite_path: SelectionIndicator::SelectionIndicator,
        }
    }
}
