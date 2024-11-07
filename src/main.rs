use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::{
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetClientPlugin, RenetServerPlugin,
};
use client::client::Client;
use plugins::{
    client::ClientPlugin, event_handlers::EventHandlersPlugin, events::EventsPlugin,
    groups::developer_plugin_group::DeveloperPluginGroup, resources::ResourcesPlugin,
    running::RunningPlugin, server::ServerPlugin, start::StartPlugin,
    user_interface::UserInterfacePlugin,
};
use server::server::Server;

mod assets;
mod client;
mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod server;
mod systems;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = std::env::args().collect();
    let exec_type = &args[1];
    let is_host = match exec_type.as_str() {
        "client" => false,
        "server" => true,
        _ => panic!("Invalid argument, must be \"client\" or \"server\"."),
    };

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Superior Space Domination".to_string(),
                    resolution: (640.0, 480.0).into(),
                    resize_constraints: WindowResizeConstraints {
                        min_width: 640.0,
                        min_height: 480.0,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
        EguiPlugin,
        EventsPlugin,
        EventHandlersPlugin,
        ResourcesPlugin,
        StartPlugin,
        RunningPlugin,
        UserInterfacePlugin,
        DeveloperPluginGroup,
    ));

    if is_host {
        app.add_plugins((ServerPlugin, RenetServerPlugin, NetcodeServerPlugin));
        let (server, transport) = Server::new_renet_server();
        app.insert_resource(server).insert_resource(transport);
    } else {
        app.add_plugins((ClientPlugin, RenetClientPlugin, NetcodeClientPlugin));
        let (client, client_transport) = Client::new_renet_client();
        app.insert_resource(client)
            .insert_resource(client_transport);
    }

    app.run();
}
