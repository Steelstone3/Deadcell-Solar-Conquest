use bevy::{
    ecs::{event::EventWriter, system::Query},
    input::{ButtonInput, mouse::MouseButton},
    math::Vec2,
    prelude::{KeyCode, Res},
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
};

use crate::events::input_events::{
    MouseLeftClickEvent, MouseLeftClickModifierEvent, MouseRightClickEvent,
};

pub fn handle_left_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_button: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut left_mouse_event_writer: EventWriter<MouseLeftClickEvent>,
) {
    let cursor_world_position = get_cursor_position(window_query, camera_query);

    if let Ok(cursor_world_position) = cursor_world_position {
        let ctrl_modifier = keyboard_button.pressed(KeyCode::ControlLeft)
            || keyboard_button.pressed(KeyCode::ControlRight);

        if !ctrl_modifier {
            for button in mouse_button.get_just_pressed() {
                if *button == MouseButton::Left {
                    left_mouse_event_writer.write(MouseLeftClickEvent {
                        cursor_world_position,
                    });
                    return;
                }
            }
        }
    }
}

pub fn handle_left_click_with_modifier(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_button: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut left_mouse_modifier_event_writer: EventWriter<MouseLeftClickModifierEvent>,
) {
    let cursor_world_position = get_cursor_position(window_query, camera_query);

    if let Ok(cursor_world_position) = cursor_world_position {
        let ctrl_modifier = keyboard_button.pressed(KeyCode::ControlLeft)
            || keyboard_button.pressed(KeyCode::ControlRight);

        if ctrl_modifier {
            for button in mouse_button.get_pressed() {
                if *button == MouseButton::Left {
                    left_mouse_modifier_event_writer.write(MouseLeftClickModifierEvent {
                        cursor_world_position,
                        just_released: false,
                    });
                    return;
                }
            }
            for button in mouse_button.get_just_released() {
                if *button == MouseButton::Left {
                    left_mouse_modifier_event_writer.write(MouseLeftClickModifierEvent {
                        cursor_world_position,
                        just_released: true,
                    });
                    return;
                }
            }
        }
    }
}

pub fn handle_right_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut right_mouse_event_writer: EventWriter<MouseRightClickEvent>,
) {
    let cursor_world_position = get_cursor_position(window_query, camera_query);

    if let Ok(cursor_world_position) = cursor_world_position {
        for button in mouse_button.get_just_pressed() {
            if *button == MouseButton::Right {
                right_mouse_event_writer.write(MouseRightClickEvent {
                    cursor_world_position,
                });
            }
        }
    }
}

fn get_cursor_position(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) -> Result<Vec2, ()> {
    let Ok(window) = window_query.single() else {
        return Err(());
    };
    let Ok(camera) = camera_query.single() else {
        return Err(());
    };

    let Some(cursor_position) = window.cursor_position() else {
        return Err(());
    };

    match camera.0.viewport_to_world_2d(camera.1, cursor_position) {
        Ok(cursor_world_position) => Ok(cursor_world_position),
        Err(_) => Err(()),
    }
}
