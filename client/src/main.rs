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
use deadcell_solar_conquest_shared::resources::server_configuration_factories::{
    create_client_configuration, create_transport_configuration,
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

    let transport = match create_transport_configuration() {
        Ok(transport) => transport,
        Err(_) => return,
    };

    let client = create_client_configuration();

    app.insert_resource(client);
    app.insert_resource(transport);
    // app.insert_resource(CurrentClientId(client_id));

    // If any error is found we just panic
    // #[allow(clippy::never_loop)]
    // fn panic_on_error(error: On<NetcodeErrorEvent>) {
    //     panic!("{}", *error);
    // }

    // app.add_observer(panic_on_error);

    app.add_systems(Startup, debug_connection_status);
    app.add_systems(Update, receive_server_message_system);

    app.run();
}

fn receive_server_message_system(mut client: ResMut<RenetClient>) {
    println!("Hi");

    if !client.is_connected() {
        return;
    }

    // Read all pending messages on the ReliableOrdered channel
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Convert the raw bytes back into a string
        if let Ok(text) = String::from_utf8(message.to_vec()) {
            println!("Received message from server: {}", text);
        }
    }
}

fn debug_connection_status(client: Res<RenetClient>) {
    if client.is_connecting() {
        println!("Connecting to server...");
    } else if client.is_connected() {
        println!("Connected!");
    } else if client.is_disconnected() {
        println!("Disconnected.");
    }
}

