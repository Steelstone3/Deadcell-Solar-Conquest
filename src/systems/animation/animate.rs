use bevy::{
    ecs::system::{Query, Res},
    time::Time,
};

use crate::queries::animation_queries::MutableAnimationQuery;

pub fn animate_sprites(time: Res<Time>, mut animations_query: Query<MutableAnimationQuery>) {
    for mut animation_query in animations_query.iter_mut() {
        animation_query.animation_timer.timer.tick(time.delta());

        if animation_query.animation_timer.timer.just_finished() {
            if let Some(texture_atlas) = animation_query.sprite.texture_atlas.as_mut() {
                texture_atlas.index += 1;
                if texture_atlas.index >= animation_query.animation_timer.frame_count {
                    texture_atlas.index = 0;
                }
            }
        }
    }
}
