use crate::systems::server::{
    send_server_messages::send_server_messages, server_sync_players::server_sync,
    server_update_positions::server_update_positions,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    prelude::resource_exists,
};
use bevy_renet::renet::RenetServer;

pub struct ServerUpdatePlugin;

impl Plugin for ServerUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (send_server_messages, server_sync, server_update_positions)
                .run_if(resource_exists::<RenetServer>),
        );
    }
}
