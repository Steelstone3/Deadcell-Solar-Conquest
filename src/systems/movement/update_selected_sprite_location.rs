use bevy::prelude::{Query, Transform, With, Without};

use crate::components::{tracking::Tracking, user_interface::SelectedSprite};

pub fn update_selected_sprite_location(
    mut selected_sprite_queries: Query<(&Tracking, &mut Transform), With<SelectedSprite>>,
    mut entity_to_follow_queries: Query<&mut Transform, Without<Tracking>>,
) {
    for mut selection_sprite_query in selected_sprite_queries.iter_mut() {
        let Ok(entity_to_follow_transform) =
            entity_to_follow_queries.get_mut(selection_sprite_query.0.entity_to_follow)
        else {
            return;
        };
        selection_sprite_query.1.translation = entity_to_follow_transform.translation;
        selection_sprite_query.1.rotation = entity_to_follow_transform.rotation;
    }
}
