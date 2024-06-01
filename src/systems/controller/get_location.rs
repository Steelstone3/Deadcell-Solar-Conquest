use bevy::transform::components::Transform;

use crate::queries::{
    camera_queries::CameraTransformOrthographicProjectionQueryItem, window_queries::WindowQueryItem,
};

// TODO This will get used for spawn location
#[allow(dead_code)]
pub fn get_cursor_location(
    transform: &mut Transform,
    cursor_position: bevy::prelude::Vec2,
    window_query: WindowQueryItem<'_>,
    camera_query: CameraTransformOrthographicProjectionQueryItem<'_>,
) {
    transform.translation.x = ((cursor_position.x - window_query.window.resolution.width() / 2.0)
        * camera_query.projection.scale)
        + camera_query.transform.translation.x;
    transform.translation.y = -((cursor_position.y
        - window_query.window.resolution.height() / 2.0)
        * camera_query.projection.scale)
        + camera_query.transform.translation.y;
}
