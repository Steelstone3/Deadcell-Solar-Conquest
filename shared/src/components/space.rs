use crate::{
    assets::sprites::space_sprites::SpaceSprites, components::size_component::SizeComponent,
    resources::constants::SPACE_SIZE,
};
use bevy::{ecs::component::Component, transform::components::Transform};
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Space {
    pub sprite_path: SpaceSprites,
    pub transform: Transform,
    pub size_component: SizeComponent,
}

impl Space {
    pub fn new(sprite_path: SpaceSprites) -> Self {
        Self {
            sprite_path,
            transform: Transform::default(),
            size_component: SizeComponent {
                size: SPACE_SIZE,
                z_index: 0.0,
            },
        }
    }
}
