use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystemConfigs,
};
use bevy_renet::client_connected;

use crate::systems::client::client_sync_players::client_sync_players;

pub struct ClientUpdatePlugin;

impl Plugin for ClientUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, client_sync_players.run_if(client_connected));
    }
}
