use bevy::{core_pipeline::core_2d::Camera2d, prelude::Commands};

// TODO Work out how to spawn a camera for each player's base
pub fn spawn_camera(
    mut commands: Commands,
    // space_station_transform_query: Query<&Transform, With<SpaceFacility>>,
) {
    commands.spawn(Camera2d);

    // let Ok(space_station_transform) = space_station_transform_query.get_single() else {
    //     return;
    // };

    // commands.spawn(Camera2dBundle {
    //     transform: space_station_transform.with_rotation(Quat::from_rotation_x(0.0)),
    //     ..default()
    // });
}
