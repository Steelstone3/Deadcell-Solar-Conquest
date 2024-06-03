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

use crate::events::{
    mouse_click_event::MouseClickEvent, mouse_right_click_event::MouseRightClickEvent,
};

pub fn handle_mouse_input(
    mut mouse_buttons_events: EventReader<MouseButtonInput>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut left_mouse_event_writer: EventWriter<MouseClickEvent>,
    mut right_mouse_event_writer: EventWriter<MouseRightClickEvent>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok(camera) = camera_query.get_single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Some(cursor_world_pos) = camera.0.viewport_to_world_2d(camera.1, cursor_pos) else {
        return;
    };

    for event in mouse_buttons_events.read() {
        if event.state.is_pressed() {
            if event.button == MouseButton::Left {
                left_mouse_event_writer.send(MouseClickEvent {
                    cursor_world_position: cursor_world_pos,
                });
            } else if event.button == MouseButton::Right {
                right_mouse_event_writer.send(MouseRightClickEvent {
                    cursor_world_position: cursor_world_pos,
                });
            }
        }
    }
}
