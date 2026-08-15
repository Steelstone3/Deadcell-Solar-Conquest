use bevy::{
    ecs::system::{Query, Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    time::Time,
};

use crate::{
    queries::camera_queries::MutableCameraTransformQuery,
    resources::{camera_settings::CameraSettings, keybindings::KeyBindings},
};

pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera_settings: ResMut<CameraSettings>,
    mut cameras: Query<MutableCameraTransformQuery>,
    keybindings: Res<KeyBindings>,
) {
    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };
    let mut left_right = 0.0;
    let mut up_down = 0.0;

    // Inputs
    for key in keybindings.camera_up.iter() {
        if input.pressed(*key) {
            up_down += 1.0;
            break;
        }
    }
    for key in keybindings.camera_right.iter() {
        if input.pressed(*key) {
            left_right += 1.0;
            break;
        }
    }
    for key in keybindings.camera_down.iter() {
        if input.pressed(*key) {
            up_down -= 1.0;
            break;
        }
    }
    for key in keybindings.camera_left.iter() {
        if input.pressed(*key) {
            left_right -= 1.0;
            break;
        }
    }
    let is_diagonal = (up_down != 0.0) && (left_right != 0.0);
    let camera_speed =
        calculate_camera_speed(camera_settings, &keybindings, &input, &time, is_diagonal);
    camera.transform.translation.x += left_right * camera_speed;
    camera.transform.translation.y += up_down * camera_speed;
}

pub fn calculate_camera_speed(
    mut camera_settings: ResMut<CameraSettings>,
    keybindings: &Res<KeyBindings>,
    input: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
    is_diagonal: bool,
) -> f32 {
    for key in keybindings.camera_slow.iter() {
        if input.pressed(*key) {
            camera_settings.is_camera_slow = true;
            break;
        } else {
            camera_settings.is_camera_slow = false;
        }
    }
    for key in keybindings.camera_fast.iter() {
        if input.pressed(*key) {
            camera_settings.is_camera_fast = true;
            break;
        } else {
            camera_settings.is_camera_fast = false;
        }
    }
    let mut new_camera_speed = camera_settings.camera_base_speed;
    if camera_settings.is_camera_slow {
        new_camera_speed = (camera_settings.camera_slow_speed) * time.delta_secs();
    } else if camera_settings.is_camera_fast {
        new_camera_speed = (camera_settings.camera_fast_speed) * time.delta_secs();
    } else {
        new_camera_speed *= time.delta_secs();
    }
    if is_diagonal {
        calculate_diagonal_camera_speed(new_camera_speed)
    } else {
        new_camera_speed
    }
}

fn calculate_diagonal_camera_speed(camera_speed: f32) -> f32 {
    let diagonal_speed_squared = (camera_speed * camera_speed) + (camera_speed * camera_speed);
    let diagonal_speed = (diagonal_speed_squared).sqrt();

    camera_speed * (camera_speed / diagonal_speed)
}
