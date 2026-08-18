use crate::plugins::{
    client_start_plugin::ClientStartPlugin, client_update_plugin::ClientUpdatePlugin,
    event_handlers_plugin::EventHandlersPlugin, events_plugin::EventsPlugin,
    resources_plugin::ResourcesPlugin, running_plugin::RunningPlugin,
    user_interface_plugin::UserInterfacePlugin,
};
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_renet::{RenetClientPlugin, netcode::NetcodeClientPlugin};

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
        EguiPlugin::default(),
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

    app.run();
}
