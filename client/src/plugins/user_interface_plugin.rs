use crate::systems::user_interface::layouts::user_interface_menu::user_interface_menu;
use bevy::app::Plugin;
use bevy_egui::EguiPrimaryContextPass;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(EguiPrimaryContextPass, user_interface_menu);
    }
}
