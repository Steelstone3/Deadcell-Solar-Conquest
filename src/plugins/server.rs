use bevy::{
    app::{Plugin, Update},
    prelude::{resource_exists, IntoSystemConfigs},
};
use bevy_renet::renet::RenetServer;

use crate::systems::server::{
    server_sync_players::server_sync, server_update::server_update,
    server_update_positions::server_update_positions,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (server_update, server_sync, server_update_positions)
                .run_if(resource_exists::<RenetServer>),
        );
    }
}
