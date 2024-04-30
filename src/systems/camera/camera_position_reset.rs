use crate::queries::camera_queries::MutableCameraTransformQuery;
use bevy::{
    ecs::system::{Query, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec3,
};

pub fn camera_position_reset(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut cameras: Query<MutableCameraTransformQuery>,
) {
    let Ok(mut camera) = cameras.get_single_mut() else {
        return;
    };

    let is_reset_camera_position_pressed =
        input.clear_just_pressed(KeyCode::Home) || input.clear_just_pressed(KeyCode::KeyC);

    if is_reset_camera_position_pressed {
        let mut transform = camera.transform;
        transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);

        camera.transform = transform;
    }
}
