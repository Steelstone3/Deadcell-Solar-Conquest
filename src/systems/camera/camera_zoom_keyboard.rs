use crate::{
    queries::camera_queries::MutableCameraOrthographicProjectionQuery,
    resources::camera_settings::CameraSettings,
};
use bevy::{
    ecs::system::{Query, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    render::camera::Projection,
};
use float_lerp::lerp;

pub fn camera_zoom_keyboard(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut cameras: Query<MutableCameraOrthographicProjectionQuery>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };

    let is_zoom_in_pressed = input.clear_just_pressed(KeyCode::Equal);
    let is_zoom_out_pressed = input.clear_just_pressed(KeyCode::Minus);
    let is_zoom_reset_pressed = input.clear_just_pressed(KeyCode::Backspace);

    if is_zoom_in_pressed {
        camera_settings.current_zoom = (camera_settings.current_zoom * camera_settings.zoom_out
            / camera_settings.zoom_speed)
            .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
    }
    if is_zoom_out_pressed {
        camera_settings.current_zoom =
            (camera_settings.current_zoom * camera_settings.zoom_out * camera_settings.zoom_speed)
                .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
    }
    if is_zoom_reset_pressed {
        camera_settings.current_zoom = 1.0;
    }

    if let Projection::Orthographic(orthographic_projection) = &mut *camera.projection {
        orthographic_projection.scale = lerp(
            orthographic_projection.scale,
            camera_settings.current_zoom,
            0.05,
        );
    }
}
