use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::Query,
    },
    input::mouse::{MouseButton, MouseButtonInput},
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
};

use crate::events::mouse_click_event::MouseClickEvent;

pub fn select(
    mut mouse_buttons_events: EventReader<MouseButtonInput>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut select_event_writer: EventWriter<MouseClickEvent>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok(camera) = camera_query.get_single() else {
        return;
    };

    for event in mouse_buttons_events.read() {
        if event.state.is_pressed() && event.button == MouseButton::Left {
            let Some(cursor_pos) = window.cursor_position() else {
                return;
            };

            let Some(cursor_world_pos) = camera.0.viewport_to_world_2d(camera.1, cursor_pos) else {
                return;
            };

            select_event_writer.send(MouseClickEvent {
                cursor_world_position: cursor_world_pos,
            });
        }
    }
}
