use bevy::app::Plugin;
use deadcell_solar_conquest_shared::resources::{game_settings::GameSettings, lobby::ServerLobby};

pub struct GameConfigurationPlugin;

impl Plugin for GameConfigurationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<GameSettings>();
        app.init_resource::<ServerLobby>();
    }
}
