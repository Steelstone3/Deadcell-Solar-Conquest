use bevy::app::{Plugin, Startup, Update};

use crate::systems::{
    spawning::spawner::spawner,
    user_interface::{
        interactions::{
            deselect_all::deselect_all,
            select_space_facility_space_button::select_space_facility_spawn_button,
            select_spawn_menu_button::select_starship_spawn_menu_button,
            select_starship_spawn_button::select_starship_spawn_button,
        },
        layouts::{
            despawn_spawn_sub_menu::despawn_sub_menus, spawn_menu::spawn_menu,
            spawn_sub_menu::spawn_sub_menu,
        },
    },
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_menu);
        app.add_systems(Update, spawn_sub_menu);
        app.add_systems(Update, spawner);
        app.add_systems(
            Update,
            (
                select_starship_spawn_menu_button,
                select_starship_spawn_button,
                select_space_facility_spawn_button,
                despawn_sub_menus,
                deselect_all,
            ),
        );
    }
}
