use bevy::{
    math::Vec3,
    prelude::{EventReader, Query, Res, Transform},
};

use crate::{
    components::user_interface::controllable::Movement, events::input_events::MouseRightClickEvent,
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

//sets the selected controllable target location to where the player right clicked
pub fn set_single_destination(
    mut right_mouse_event_reader: EventReader<MouseRightClickEvent>,
    selection_resource: Res<SpawnMenuSelection>,
    // TODO make a query
    mut selected_moveable_queries: Query<(&mut Movement, &Transform)>,
) {
    for event in right_mouse_event_reader.read() {
        if selection_resource.selection != SpawnSelection::None {
            let Ok(mut selected_entity) =
                selected_moveable_queries.get_mut(selection_resource.selected_entity)
            else {
                return;
            };

            selected_entity.0.target_location = Vec3::new(
                event.cursor_world_position.x,
                event.cursor_world_position.y,
                selected_entity.1.translation.z,
            );
        }
    }
}

//sets the selected controllable target location to where the player right clicked
pub fn set_multiple_destination(
    mut right_mouse_event_reader: EventReader<MouseRightClickEvent>,
    selection_resource: Res<SpawnMenuSelection>,
    // TODO make a query
    mut selected_moveable_queries: Query<(&mut Movement, &Transform)>,
) {
    for event in right_mouse_event_reader.read() {
        if selection_resource.selection != SpawnSelection::None {
            for selected in selection_resource.selected_entities {
                let Ok(mut selected_entity) = selected_moveable_queries.get_mut(selected) else {
                    return;
                };

                selected_entity.0.target_location = Vec3::new(
                    event.cursor_world_position.x,
                    event.cursor_world_position.y,
                    selected_entity.1.translation.z,
                );
            }
        }
    }
}
