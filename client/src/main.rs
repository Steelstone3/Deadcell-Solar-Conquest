use crate::plugins::{
    client_start_plugin::ClientStartPlugin, events_plugin::EventsPlugin,
    resources_plugin::ResourcesPlugin, running_plugin::RunningPlugin,
    startup_plugin::StartupPlugin, user_interface_plugin::UserInterfacePlugin,
};
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::{
    RenetClient, RenetClientPlugin, netcode::NetcodeClientPlugin, renet::DefaultChannel,
};
use deadcell_solar_conquest_shared::{
    messages::server_messages::ServerMessages, plugins::glue_plugin::GluePlugin,
};

mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod systems;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Deadcell Solar Conquest".to_string(),
                    resolution: WindowResolution::new(640, 480),
                    resize_constraints: WindowResizeConstraints {
                        min_width: 640.0,
                        min_height: 480.0,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
        EguiPlugin::default(),
        RenetClientPlugin,
        NetcodeClientPlugin,
        RepliconPlugins,
        GluePlugin,
        ClientStartPlugin,
        EventsPlugin,
        ResourcesPlugin,
        UserInterfacePlugin,
        StartupPlugin,
        RunningPlugin,
    ));

    app.add_systems(Update, receive_server_message_system);
    app.add_systems(Update, send_client_message_system);

    app.run();
}

// TODO example
fn receive_server_message_system(mut client: ResMut<RenetClient>) {
    if !client.is_connected() {
        // println!("Client disconnected");
        return;
    }

    while let Some(message_bytes) = client.receive_message(DefaultChannel::ReliableOrdered) {
        match bincode_next::serde::decode_from_slice::<ServerMessages, _>(
            &message_bytes,
            bincode_next::config::standard(),
        ) {
            Ok(message) => match message.0 {
                ServerMessages::SpawnPlayer { player } => {}
                ServerMessages::RemovePlayer { id } => {}
                ServerMessages::SpawnSpace { space } => {
                    println!("Spawn space message recieved")
                }
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
            },
            Err(_) => println!("Failed to deserialize message from server"),
        }

        if let Ok(text) = String::from_utf8(message_bytes.to_vec()) {
            // println!("Received message from server: {}", text);
        }
    }
}

// TODO move to connection configuration system
fn send_client_message_system(mut client: ResMut<RenetClient>) {
    if !client.is_connected() {
        // println!("Client disconnected");
        return;
    }

    // let message = "client message";
    // client.send_message(DefaultChannel::ReliableOrdered, message.as_bytes().to_vec());

    // println!("I am a message being sent from the client to the server");
}
