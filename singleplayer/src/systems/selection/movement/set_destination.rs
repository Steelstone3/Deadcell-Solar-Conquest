use crate::{
    components::user_interface::{controllable::Movement, spawn_selection::SpawnSelection},
    events::input_events::MouseRightClickEvent,
    resources::spawn_menu_selection::SpawnMenuSelection,
};
use bevy::{
    ecs::message::MessageReader,
    math::Vec3,
    prelude::{Query, Res, Transform},
};

//sets the selected controllable target location to where the player right clicked
pub fn set_single_destination(
    mut right_mouse_event_reader: MessageReader<MouseRightClickEvent>,
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
    mut right_mouse_event_reader: MessageReader<MouseRightClickEvent>,
    selection_resource: Res<SpawnMenuSelection>,
    // TODO make a query
    mut selected_moveable_queries: Query<(&mut Movement, &Transform)>,
) {
    for event in right_mouse_event_reader.read() {
        if selection_resource.selection != SpawnSelection::None {
            for selected in &selection_resource.selected_entities {
                let Ok(mut selected_entity) = selected_moveable_queries.get_mut(*selected) else {
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
