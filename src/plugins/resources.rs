use bevy::app::Plugin;

use crate::resources::{
    camera_settings::CameraSettings, faction::PlayerFaction, game_settings::GameSettings,
    keybindings::KeyBindings, spawn_menu_selection::SpawnMenuSelection,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraSettings::default())
            .insert_resource(GameSettings::default())
            .insert_resource(SpawnMenuSelection::default())
            .insert_resource(PlayerFaction::default())
            .insert_resource(KeyBindings::default());
    }
}
