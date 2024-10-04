use bevy::{
    app::{Plugin, Startup},
    prelude::IntoSystemConfigs,
};

use crate::systems::{
    camera::spawn_camera::spawn_camera,
    spawning::{
        spawn_space_stations::spawn_space_stations, spawn_resource_planets::spawn_resource_planets,
        spawn_space::spawn_space, spawn_starter_spaceship::spawn_starter_spaceship,
        spawn_suns::spawn_suns,
    },
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            (
                spawn_space,
                spawn_camera,
                spawn_suns,
                spawn_space_stations,
                spawn_resource_planets.after(spawn_suns),
                (spawn_starter_spaceship, spawn_camera).after(spawn_space_stations),
            ),
        );
    }
}
