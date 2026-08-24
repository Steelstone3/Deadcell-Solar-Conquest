use crate::systems::spawn_universe::spawn_space::spawn_space;
use bevy::app::{App, Plugin, Startup};

pub struct SpawnGameUniversePlugin;

impl Plugin for SpawnGameUniversePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_space);
    }
}
