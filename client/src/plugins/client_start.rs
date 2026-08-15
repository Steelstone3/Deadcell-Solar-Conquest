use bevy::app::{Plugin, Startup};

use crate::systems::{
    camera::spawn_camera::spawn_camera, client::connect_to_server::connect_to_server,
};

pub struct ClientStartPlugin;

impl Plugin for ClientStartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, connect_to_server);
    }
}
