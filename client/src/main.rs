use crate::plugins::{
    client_start_plugin::ClientStartPlugin, client_update_plugin::ClientUpdatePlugin,
    event_handlers_plugin::EventHandlersPlugin, events_plugin::EventsPlugin,
    resources_plugin::ResourcesPlugin, running_plugin::RunningPlugin,
    user_interface_plugin::UserInterfacePlugin,
};
use bevy::{prelude::*, window::WindowResolution};
use bevy_renet::{
    RenetClient, RenetClientPlugin, netcode::NetcodeClientPlugin, renet::DefaultChannel,
};
use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientTransport, NetcodeError},
    renet::ConnectionConfig,
};
use std::{net::UdpSocket, time::SystemTime};

mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod systems;

const SERVER_ADDRESS: &str = "127.0.0.1:5000";
const CLIENT_ADDRESS: &str = "127.0.0.1:0";

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
                    title: "Superior Space Domination".to_string(),
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
        ClientStartPlugin,
        ClientUpdatePlugin,
        EventsPlugin,
        EventHandlersPlugin,
        ResourcesPlugin,
        UserInterfacePlugin,
        RunningPlugin,
    ));

    let transport = match create_client_transport_configuration() {
        Ok(transport) => transport,
        Err(_) => return,
    };

    let client = create_client_configuration();

    app.insert_resource(client);
    app.insert_resource(transport);

    app.add_systems(Startup, client_connection_status);
    app.add_systems(Update, receive_server_message_system);

    app.run();
}

// TODO move to connection configuration system
fn receive_server_message_system(mut client: ResMut<RenetClient>) {
    if !client.is_connected() {
        return;
    }

    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(text) = String::from_utf8(message.to_vec()) {
            println!("Received message from server: {}", text);
        }
    }
}

// TODO move to debug plugin
fn client_connection_status(client: Res<RenetClient>) {
    if client.is_connecting() {
        println!("Connecting to server...");
    } else if client.is_connected() {
        println!("Connected!");
    } else if client.is_disconnected() {
        println!("Disconnected.");
    }
}

// TODO move to connection configuration system
fn create_client_configuration() -> RenetClient {
    RenetClient::new(ConnectionConfig {
        client_channels_config: DefaultChannel::config(),
        server_channels_config: DefaultChannel::config(),
        ..Default::default()
    })
}

// TODO move to connection configuration system
fn create_client_transport_configuration() -> Result<NetcodeClientTransport, NetcodeError> {
    let authentication = ClientAuthentication::Unsecure {
        server_addr: match SERVER_ADDRESS.parse() {
            Ok(addr) => addr,
            Err(_) => return Err(NetcodeError::ClientNotConnected),
        },
        client_id: 0,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind(CLIENT_ADDRESS)?;
    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => std::time::Duration::from_millis(0),
    };

    NetcodeClientTransport::new(current_time, authentication, socket)
}
