use crate::plugins::{
    game_configuration_plugin::GameConfigurationPlugin, server_start_plugin::ServerStartPlugin,
    spawn_game_universe_plugin::StartupPlugin,
};
use bevy::{
    MinimalPlugins, app::App , ecs::{
        lifecycle::Add,
        observer::On,
        system::{Commands, ResMut},
    }, state::app::StatesPlugin,
};

use bevy_replicon::{RepliconPlugins, shared::backend::connected_client::ConnectedClient};
use bevy_replicon_renet::{RenetServer, RenetServerPlugin, netcode::NetcodeServerPlugin, renet::DefaultChannel};
use deadcell_solar_conquest_shared::plugins::glue_plugin::GluePlugin;

mod plugins;
mod resources;
mod systems;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
fn main() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        StatesPlugin,
        RenetServerPlugin,
        NetcodeServerPlugin,
        RepliconPlugins,
        GluePlugin,
        ServerStartPlugin,
        StartupPlugin,
        GameConfigurationPlugin,
    ));
    app.add_observer(on_client_connected);

    // app.add_systems(Update, send_server_message_system);
    // app.add_systems(Update, recieve_server_message_system);

    app.run();
}

fn on_client_connected(
    trigger: On<Add, ConnectedClient>, // Fired when Replicon registers a connected client entity
    mut commands: Commands,
) {
    let client_entity = trigger.entity;
    println!("New client connected on entity: {:?}", client_entity);

    // Spawn tiles or player data for this client here
}

// TODO example
fn send_server_message_system(mut server: ResMut<RenetServer>) {
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message".as_bytes());

    println!("I am a message being sent from the server to the client");
}

// TODO example
fn recieve_server_message_system(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id() {
        if let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            println!(
                "I am a message being received from the client: {}",
                String::from_utf8_lossy(&message)
            );
        }
    }
}
