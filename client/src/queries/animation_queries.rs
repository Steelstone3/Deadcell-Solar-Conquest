use bevy::{ecs::query::QueryData, sprite::Sprite};
use deadcell_solar_conquest_shared::components::client::user_interface::animation_timer::AnimationTimer;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableAnimationQuery {
    pub animation_timer: &'static mut AnimationTimer,
    pub sprite: &'static mut Sprite,
}
