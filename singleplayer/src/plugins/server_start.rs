use bevy::{
    app::{Plugin, Startup},
    ecs::schedule::IntoScheduleConfigs,
};

use crate::systems::{
    camera::spawn_camera::spawn_camera,
    spawning::{
        spawn_resource_planets::spawn_resource_planets, spawn_space::spawn_space,
        spawn_stars::spawn_stars, spawn_starter_starbase::spawn_starter_starbase,
    },
};

pub struct ServerStartPlugin;

impl Plugin for ServerStartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_space);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, spawn_starter_starbase);
        app.add_systems(Startup, spawn_stars);
        app.add_systems(Startup, spawn_resource_planets.after(spawn_stars));
    }
}
