use bevy::app::{Plugin, Update};

use crate::systems::spawning::spawn_space::spawn_space;

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, spawn_space);
    }
}
