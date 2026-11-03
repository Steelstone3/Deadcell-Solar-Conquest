use crate::resources::{server_address::ServerAddress, spawn_menu_selection::SpawnMenuSelection};
use bevy::app::Plugin;
use deadcell_solar_conquest_shared::resources::player::Player;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<Player>();
        app.init_resource::<ServerAddress>();
        app.init_resource::<SpawnMenuSelection>();
    }
}
