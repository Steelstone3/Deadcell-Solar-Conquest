use crate::plugins::{
    client_start_plugin::ClientStartPlugin, resources_plugin::ResourcesPlugin,
    running_plugin::RunningPlugin, startup_plugin::StartupPlugin,
    user_interface_plugin::UserInterfacePlugin,
};
use bevy::{prelude::*, window::WindowResolution};
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::{RenetClient, RenetClientPlugin, netcode::NetcodeClientPlugin, renet::DefaultChannel};
use deadcell_solar_conquest_shared::plugins::glue_plugin::GluePlugin;

mod components;
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
        // EguiPlugin::default(),
        RenetClientPlugin,
        NetcodeClientPlugin,
        RepliconPlugins,
        GluePlugin,
        ClientStartPlugin,
        ResourcesPlugin,
        UserInterfacePlugin,
        StartupPlugin,
        RunningPlugin,
    ));

    // app.add_systems(Update, receive_server_message_system);
    // app.add_systems(Update, send_client_message_system);

    app.run();
}

// TODO example
fn receive_server_message_system(mut client: ResMut<RenetClient>) {
    if !client.is_connected() {
        println!("Client disconnected");
        return;
    }

    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(text) = String::from_utf8(message.to_vec()) {
            println!("Received message from server: {}", text);
        }
    }
}

// TODO example
fn send_client_message_system(mut client: ResMut<RenetClient>) {
    if !client.is_connected() {
        println!("Client disconnected");
        return;
    }

    let message = "client message";
    client.send_message(DefaultChannel::ReliableOrdered, message.as_bytes().to_vec());

    println!("I am a message being sent from the client to the server");
}
