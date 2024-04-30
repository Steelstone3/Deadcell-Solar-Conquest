use bevy::prelude::*;
use plugins::{
    event_handlers::EventHandlersPlugin, events::EventsPlugin, resources::ResourcesPlugin,
    running::RunningPlugin, start::StartPlugin,
};

mod assets;
mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins((
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
            EventsPlugin,
            EventHandlersPlugin,
            ResourcesPlugin,
            StartPlugin,
            RunningPlugin,
            // UserInterfacePlugin,
        ))
        .run();
}
