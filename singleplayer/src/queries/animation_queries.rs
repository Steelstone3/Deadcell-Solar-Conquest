use crate::components::user_interface::animation_timer::AnimationTimer;
use bevy::{ecs::query::QueryData, sprite::Sprite};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableAnimationQuery {
    pub animation_timer: &'static mut AnimationTimer,
    pub sprite: &'static mut Sprite,
}
