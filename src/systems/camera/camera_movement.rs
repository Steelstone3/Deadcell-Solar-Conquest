use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, ButtonInput},
    time::Time,
};

use crate::{
    queries::camera_queries::MutableCameraTransformQuery, resources::keybindings::KeyBindings,
};

pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<MutableCameraTransformQuery>,
    keybindings: Res<KeyBindings>,
) {
    let Ok(mut camera) = cameras.get_single_mut() else {
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
    let camera_speed = calculate_camera_speed(&500.0, &keybindings, &input, &time, is_diagonal);
    camera.transform.translation.x += left_right * camera_speed;
    camera.transform.translation.y += up_down * camera_speed;
}

pub fn calculate_camera_speed(
    base_camera_speed: &f32,
    keybindings: &Res<KeyBindings>,
    input: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
    is_diagonal: bool,
) -> f32 {
    let mut camera_slow = false;
    for key in keybindings.camera_slow.iter() {
        if input.pressed(*key) {
            camera_slow = true;
            break;
        }
    }
    let mut camera_fast = false;
    for key in keybindings.camera_fast.iter() {
        if input.pressed(*key) {
            camera_fast = true;
            break;
        }
    }
    let mut new_camera_speed = *base_camera_speed;
    if camera_slow {
        new_camera_speed = (new_camera_speed / 2.0) * time.delta_seconds();
    } else if camera_fast {
        new_camera_speed = (new_camera_speed * 2.0) * time.delta_seconds();
    } else {
        new_camera_speed *= time.delta_seconds();
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
