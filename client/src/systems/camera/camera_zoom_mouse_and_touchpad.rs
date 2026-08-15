use crate::{
    queries::camera_queries::MutableCameraOrthographicProjectionQuery,
    resources::camera_settings::CameraSettings,
};
use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseWheel},
    render::camera::Projection,
};
use float_lerp::lerp;

pub fn camera_zoom_mouse_and_touchpad(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut cameras: Query<MutableCameraOrthographicProjectionQuery>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };

    for mouse_wheel_event in mouse_wheel_events.read() {
        if mouse_wheel_event.y < 0.0 {
            camera_settings.current_zoom = (camera_settings.current_zoom
                * camera_settings.zoom_in
                * camera_settings.zoom_speed)
                .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
        } else if mouse_wheel_event.y > 0.0 {
            camera_settings.current_zoom = (camera_settings.current_zoom
                * camera_settings.zoom_out
                / camera_settings.zoom_speed)
                .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
        }
    }

    if input.clear_just_pressed(KeyCode::KeyR) {
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
