use bevy::{ecs::component::Component, math::Vec2};

use crate::{
    assets::{
        images::space_facility_sprite::SpaceFacilitySprite,
        user_interace::icons::space_facility_icons::SpaceFacilityIcon,
    },
    resources::constants::STATION_SIZE,
};

use super::size_component::SizeComponent;

const SPACE_FACILITY_SIZE: Vec2 = Vec2::new(STATION_SIZE, STATION_SIZE);

#[derive(Component, Clone, Copy)]
pub struct SpaceFacility {
    pub sprite_path: SpaceFacilitySprite,
    pub size_component: SizeComponent,
}

impl SpaceFacility {
    pub fn new_from_icon(starship_icon: SpaceFacilityIcon) -> SpaceFacility {
        let sprite_path = SpaceFacilitySprite::sprite_convert_from(starship_icon);

        Self {
            sprite_path,
            size_component: SizeComponent {
                size: SPACE_FACILITY_SIZE,
                z_index: 3.0,
            },
        }
    }
}
