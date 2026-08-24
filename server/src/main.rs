use crate::plugins::{
    events_plugin::EventsPlugin, game_configuration_plugin::GameConfigurationPlugin, server_start_plugin::ServerStartPlugin, spawn_game_universe_plugin::SpawnGameUniversePlugin,
};
use bevy::{
    MinimalPlugins,
    app::{App, Update},
    ecs::system::ResMut,
};
use bevy_renet::{
    RenetServer, RenetServerPlugin, netcode::NetcodeServerPlugin, renet::DefaultChannel,
};

mod plugins;
mod resources;
mod systems;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);
    app.add_plugins(ServerStartPlugin);
    app.add_plugins(EventsPlugin);
    app.add_plugins(SpawnGameUniversePlugin);
    app.add_plugins(GameConfigurationPlugin);

    app.add_systems(Update, send_server_message_system);
    app.add_systems(Update, recieve_server_message_system);

    app.run();
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
