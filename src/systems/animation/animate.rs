use bevy::{
    ecs::system::{Query, Res},
    time::Time,
};

use crate::queries::animation_queries::MutableAnimationQuery;

pub fn animate_sprites(time: Res<Time>, mut animations_query: Query<MutableAnimationQuery>) {
    for mut animation_query in animations_query.iter_mut() {
        animation_query.animation_timer.timer.tick(time.delta());

        if animation_query.animation_timer.timer.just_finished() {
            animation_query.texture_atlas.index += 1;
            if animation_query.texture_atlas.index >= animation_query.animation_timer.frame_count {
                animation_query.texture_atlas.index = 0;
            }
        }
    }
}
