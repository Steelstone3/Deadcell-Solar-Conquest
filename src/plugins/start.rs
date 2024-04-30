use bevy::app::{Plugin, Startup};

use crate::systems::{
    camera::spawn_camera::spawn_camera,
    spawning::{
        spawn_player_base::spawn_space_stations, spawn_space::spawn_space, spawn_sun::spawn_sun,
    },
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            (spawn_space, spawn_camera, spawn_sun, spawn_space_stations),
        );
    }
}
