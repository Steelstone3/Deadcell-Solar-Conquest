use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_renet::{
    RenetClientPlugin, RenetServerPlugin,
    netcode::{NetcodeClientPlugin, NetcodeServerPlugin},
};
use plugins::{
    event_handlers::EventHandlersPlugin, events::EventsPlugin, resources::ResourcesPlugin,
    running::RunningPlugin, server_start::ServerStartPlugin, user_interface::UserInterfacePlugin,
};

use crate::plugins::client_start::ClientStartPlugin;

mod assets;
mod client;
mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod server;
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
        EguiPlugin::default(),
        EventsPlugin,
        EventHandlersPlugin,
        ResourcesPlugin,
        UserInterfacePlugin,
        RunningPlugin,
    ));
    client_server_setup(&mut app);
    app.run();
}

fn client_server_setup(app: &mut App) {
    let args: Vec<String> = std::env::args().collect();
    let is_host = args.contains(&"server".to_string());

    if is_host {
        app.add_plugins((
            ServerStartPlugin,
            // ServerUpdatePlugin,
            RenetServerPlugin,
            NetcodeServerPlugin,
        ));
        // let (server, transport) = Server::new_renet_server();
        // app.insert_resource(server).insert_resource(transport);
    } else {
        app.add_plugins((
            ClientStartPlugin,
            // ClientUpdatePlugin,
            RenetClientPlugin,
            NetcodeClientPlugin,
        ));
        // let (client, client_transport) = Client::new_renet_client();
        // app.insert_resource(client)
            // .insert_resource(client_transport);
    }
}
