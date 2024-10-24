use crate::systems::{
    spawning::spawner::spawner,
    user_interface::{
        interactions::clear_all_selected::clear_all_selected,
        layouts::spawn_menu::spawn_menu,
        sprite_selection::{set_selection_type, sprite_selection},
    },
};
use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystem,
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_menu);
        app.add_systems(Update, sprite_selection.pipe(set_selection_type));
        app.add_systems(Update, (clear_all_selected, spawner));
    }
}
