use crate::systems::{
    selection::{
        draw_multiple_selection_box::draw_multiple_selection_box,
        select_multiple_sprites::select_multiple_sprites,
        select_sprite::{select_sprite, set_selection_type},
    },
    spawning::spawner::spawner,
    user_interface::{
        interactions::clear_all_selected::clear_all_selected, layouts::spawn_menu::spawn_menu,
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
        app.add_systems(Update, select_sprite.pipe(set_selection_type));
        app.add_systems(Update, select_multiple_sprites);
        app.add_systems(Update, draw_multiple_selection_box);
        app.add_systems(Update, clear_all_selected);
        app.add_systems(Update, spawner);
    }
}
