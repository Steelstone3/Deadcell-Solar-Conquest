use bevy::{ecs::query::QueryData, sprite::TextureAtlas};

use crate::components::animation_timer::AnimationTimer;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableAnimationQuery {
    pub animation_timer: &'static mut AnimationTimer,
    pub texture_atlas: &'static mut TextureAtlas,
}
