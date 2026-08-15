use bevy::{camera::Camera2d, prelude::Commands};

// TODO Work out how to spawn a camera for each player's base
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
