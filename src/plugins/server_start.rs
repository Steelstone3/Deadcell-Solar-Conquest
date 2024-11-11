use bevy::{
    app::{Plugin, Startup},
    prelude::IntoSystemConfigs,
};

use crate::systems::{
    camera::spawn_camera::spawn_camera,
    spawning::{
        spawn_resource_planets::spawn_resource_planets, spawn_space::spawn_space,
        spawn_space_stations::spawn_space_stations,
        spawn_starter_spaceship::spawn_starter_spaceship, spawn_suns::spawn_suns,
    },
};

pub struct ServerStartPlugin;

impl Plugin for ServerStartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_space);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, spawn_suns);
        app.add_systems(Startup, spawn_space_stations.after(spawn_suns));
        app.add_systems(Startup, spawn_resource_planets.after(spawn_suns));
        app.add_systems(Startup, spawn_starter_spaceship.after(spawn_space_stations));
    }
}
