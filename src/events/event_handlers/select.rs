use bevy::{
    ecs::{event::EventWriter, system::Query},
    input::{mouse::MouseButton, ButtonInput},
    prelude::{KeyCode, Res},
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
};

use crate::events::{
    mouse_click_event::MouseClickEvent, mouse_right_click_event::MouseRightClickEvent,
};

pub fn handle_mouse_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut left_mouse_event_writer: EventWriter<MouseClickEvent>,
    mut right_mouse_event_writer: EventWriter<MouseRightClickEvent>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
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

    for button in mouse_buttons.get_pressed() {
        if *button == MouseButton::Left {
            left_mouse_event_writer.send(MouseClickEvent {
                cursor_world_position: cursor_world_pos,
                ctrl_modifier: keyboard_buttons.pressed(KeyCode::ControlLeft),
                just_released: false,
            });
            return;
        } else if *button == MouseButton::Right {
            right_mouse_event_writer.send(MouseRightClickEvent {
                cursor_world_position: cursor_world_pos,
            });
            return;
        }
    }

    for button in mouse_buttons.get_just_released() {
        if *button == MouseButton::Left {
            left_mouse_event_writer.send(MouseClickEvent {
                cursor_world_position: cursor_world_pos,
                ctrl_modifier: keyboard_buttons.pressed(KeyCode::ControlLeft),
                just_released: true,
            });
            return;
        }
    }
}
