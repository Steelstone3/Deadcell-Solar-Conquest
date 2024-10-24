use bevy::{
    math::Quat,
    prelude::{Camera2dBundle, Commands, Query, Transform, With},
    utils::default,
};

use crate::components::space_facility::SpaceFacility;

pub fn spawn_camera(
    mut commands: Commands,
    space_station_transform_query: Query<&Transform, With<SpaceFacility>>,
) {
    let Ok(space_station_transform) = space_station_transform_query.get_single() else {
        return;
    };

    commands.spawn(Camera2dBundle {
        transform: space_station_transform.with_rotation(Quat::from_rotation_x(0.0)),
        ..default()
    });
}
