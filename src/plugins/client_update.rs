use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
};
use bevy_renet::client_connected;

// use crate::systems::client::receive_server_messages::receive_server_messages;

pub struct ClientUpdatePlugin;

impl Plugin for ClientUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_systems(Update, receive_server_messages.run_if(client_connected));
    }
}
