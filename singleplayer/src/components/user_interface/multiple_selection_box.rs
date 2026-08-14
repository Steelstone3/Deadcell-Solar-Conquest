use bevy::{math::Vec2, prelude::Component};

use crate::assets::user_interface::selection_indicator::SelectionIndicator;

#[derive(Component, Clone, Copy)]
pub struct MultipleSelectionBox {
    pub sprite_path: SelectionIndicator,
    pub selection_area: SelectionArea,
}

impl MultipleSelectionBox {
    pub fn new(cursor_position: Vec2) -> Self {
        Self {
            sprite_path: SelectionIndicator::SelectionBox,
            selection_area: SelectionArea::new(cursor_position),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct SelectionArea {
    pub start: Vec2,
    pub end: Vec2,
}

impl SelectionArea {
    pub fn new(cursor_position: Vec2) -> Self {
        Self {
            start: cursor_position,
            end: cursor_position,
        }
    }
}
