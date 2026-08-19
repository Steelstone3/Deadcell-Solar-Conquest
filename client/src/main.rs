use crate::plugins::{
    client_start_plugin::ClientStartPlugin, client_update_plugin::ClientUpdatePlugin,
    event_handlers_plugin::EventHandlersPlugin, events_plugin::EventsPlugin,
    resources_plugin::ResourcesPlugin, running_plugin::RunningPlugin,
    user_interface_plugin::UserInterfacePlugin,
};
use bevy::{prelude::*, window::WindowResolution};
use std::{net::UdpSocket, time::SystemTime};
// use bevy_egui::EguiPlugin;
use bevy_renet::{
    RenetClient, RenetClientPlugin,
    netcode::{
        ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeErrorEvent,
    },
    renet::ConnectionConfig,
};
use deadcell_solar_conquest_shared::resources::lobby::{ClientChannel, ServerChannel};

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
        ClientStartPlugin,
        ClientUpdatePlugin,
        EventsPlugin,
        EventHandlersPlugin,
        ResourcesPlugin,
        UserInterfacePlugin,
        RunningPlugin,
    ));

    // pub const PROTOCOL_ID: u64 = 7;

    app.add_plugins(NetcodeClientPlugin);

    app.configure_sets(Update, Connected.run_if(bevy_renet::client_connected));

    let client = RenetClient::new(connection_config());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 7,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(client);
    app.insert_resource(transport);
    app.insert_resource(CurrentClientId(client_id));

    // If any error is found we just panic
    // #[allow(clippy::never_loop)]
    // fn panic_on_error(error: On<NetcodeErrorEvent>) {
    //     panic!("{}", *error);
    // }

    // app.add_observer(panic_on_error);

    app.run();
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}
