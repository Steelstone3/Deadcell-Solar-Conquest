use bevy::{
    app::{Plugin, Startup, Update},
    prelude::IntoSystemConfigs,
};
use bevy_renet::client_connected;

use crate::systems::client::{
    client_sync_players::client_sync_players, connect_to_server::connect_to_server,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, client_sync_players.run_if(client_connected));
        app.add_systems(Startup, connect_to_server);
    }
}
