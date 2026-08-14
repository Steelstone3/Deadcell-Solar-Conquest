use bevy::app::{Plugin, Startup};

use crate::systems::camera::spawn_camera::spawn_camera;

pub struct ClientStartPlugin;

impl Plugin for ClientStartPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
    }
}
