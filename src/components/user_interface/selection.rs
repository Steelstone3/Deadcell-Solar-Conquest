use crate::assets::user_interface::team::TeamSelectionSprite;
use bevy::prelude::Component;

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
