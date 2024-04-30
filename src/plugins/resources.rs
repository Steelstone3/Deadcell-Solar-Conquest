use bevy::app::Plugin;

use crate::resources::{
    camera_settings::CameraSettings, game_settings::GameSettings, keybindings::KeyBindings,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraSettings::default())
            .insert_resource(GameSettings::default())
            .insert_resource(KeyBindings::default());
    }
}
