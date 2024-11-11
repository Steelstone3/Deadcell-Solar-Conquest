use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::{
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetClientPlugin, RenetServerPlugin,
};
use plugins::{
    client_start::ClientStartPlugin, client_update::ClientUpdatePlugin,
    event_handlers::EventHandlersPlugin, events::EventsPlugin,
    groups::developer_plugin_group::DeveloperPluginGroup, resources::ResourcesPlugin,
    running::RunningPlugin, server_start::ServerStartPlugin, server_update::ServerUpdatePlugin,
    user_interface::UserInterfacePlugin,
};
use systems::{client::game_client::Client, server::game_server::Server};

mod assets;
mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod systems;

fn main() {
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
        UserInterfacePlugin,
        RunningPlugin,
        DeveloperPluginGroup,
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
            ServerUpdatePlugin,
            RenetServerPlugin,
            NetcodeServerPlugin,
        ));
        let (server, transport) = Server::new_renet_server();
        app.insert_resource(server).insert_resource(transport);
    } else {
        app.add_plugins((
            ClientStartPlugin,
            ClientUpdatePlugin,
            RenetClientPlugin,
            NetcodeClientPlugin,
        ));
        let (client, client_transport) = Client::new_renet_client();
        app.insert_resource(client)
            .insert_resource(client_transport);
    }
}
