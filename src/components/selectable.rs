use bevy::ecs::component::Component;

use crate::assets::images::selection::SelectionSprite;

#[derive(Component, Clone, Copy)]
pub struct Selection {
    pub sprite_path: SelectionSprite,
}

impl Selection {
    pub fn new(sprite_path: SelectionSprite) -> Self {
        Self { sprite_path }
    }
}
