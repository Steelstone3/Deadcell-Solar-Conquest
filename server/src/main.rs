use crate::{
    plugins::{
        game_configuration_plugin::GameConfigurationPlugin, server_start_plugin::ServerStartPlugin,
        spawn_game_universe_plugin::StartupPlugin,
    },
    systems::spawn_universe::spawn_space::spawn_space_on_client_connected,
};
use bevy::{
    MinimalPlugins,
    app::{App, PluginGroup, ScheduleRunnerPlugin, Update},
    ecs::{
        schedule::{IntoScheduleConfigs, common_conditions::resource_exists},
        system::{Commands, Res, ResMut},
    },
    state::app::StatesPlugin,
};
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::{
    RenetServer, RenetServerPlugin,
    netcode::NetcodeServerPlugin,
    renet::DefaultChannel::{self, ReliableOrdered},
};
use deadcell_solar_conquest_shared::{
    messages::server_messages::ServerMessages,
    plugins::glue_plugin::GluePlugin,
    resources::game_settings::{self, GameSettings},
};
use std::time::Duration;

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
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
        StatesPlugin,
        RenetServerPlugin,
        NetcodeServerPlugin,
        RepliconPlugins,
        GluePlugin,
        ServerStartPlugin,
        StartupPlugin,
        GameConfigurationPlugin,
    ));

    app.add_systems(
        Update,
        send_server_message_system.run_if(resource_exists::<RenetServer>),
    );
    app.add_systems(
        Update,
        receive_client_message_system.run_if(resource_exists::<RenetServer>),
    );

    app.run();
}

// TODO example
fn send_server_message_system(mut server: ResMut<RenetServer>) {
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message".as_bytes());

    // println!("I am a message being sent from the server to the client");
}

// // TODO move to connection configuration system
// fn recieve_client_message_system(mut server: ResMut<RenetServer>) {
//     for client_id in server.clients_id() {
//         if let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
//             println!(
//                 "I am a message being received from the client: {}",
//                 String::from_utf8_lossy(&message)
//             );
//         }
//     }
// }

pub fn receive_client_message_system(
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    game_settings: Res<GameSettings>,
) {
    for client_id in server.clients_id() {
        while let Some(message_bytes) =
            server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            // Deserialize messages
            match bincode_next::serde::decode_from_slice::<ServerMessages, _>(
                &message_bytes,
                bincode_next::config::standard(),
            ) {
                Ok((message, _)) => {
                    match message {
                        ServerMessages::SpawnPlayer { player } => {
                            println!(
                                "Successfully received CreatePlayer for player from client {client_id}"
                            );

                            // spawn game startup stuff
                            spawn_space_on_client_connected(
                                commands.reborrow(),
                                &game_settings,
                                &mut server,
                            );
                        }
                        // Handle other enum variants here...
                        ServerMessages::RemovePlayer { id } => {}
                        ServerMessages::SpawnSpace { space } => {}
                        ServerMessages::SpawnStar {
                            entity,
                            translation,
                        } => {}
                        ServerMessages::SpawnPlanet {
                            entity,
                            translation,
                        } => {}
                        ServerMessages::SpawnStarship {
                            entity,
                            translation,
                        } => {}
                        ServerMessages::MoveStarship {
                            entity,
                            translation,
                        } => {}
                        ServerMessages::DespawnStarship { entity } => {}
                        ServerMessages::SpawnProjectile {
                            entity,
                            translation,
                        } => {}
                        ServerMessages::DespawnProjectile { entity } => {}
                    }
                }
                Err(e) => {
                    println!("Failed to deserialize message from client {client_id}: {e}");
                }
            }
        }
    }
}
