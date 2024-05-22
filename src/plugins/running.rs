use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    events::{event_handlers::select::select, spawn_planet_event::SpawnPlanetEvent},
    systems::{
        animation::animate::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        selecting::select_selectable::select_selectable,
        spawning::spawn_resource_planets::spawn_resource_planets,
    },
};

use super::run_conditions::event_called;

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                camera_zoom_keyboard,
                camera_zoom_mouse_and_touchpad,
                camera_movement,
                camera_position_reset,
                select,
                select_selectable,
                animate_sprites,
            ),
        )
        .add_systems(
            Update,
            spawn_resource_planets.run_if(event_called::<SpawnPlanetEvent>),
        );
    }
}
