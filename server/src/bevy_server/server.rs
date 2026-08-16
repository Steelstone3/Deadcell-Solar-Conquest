use bevy::app::{PluginGroup, Update};
use bevy::log::LogPlugin;
use bevy::prelude::App;
use bevy::window::{ExitCondition, WindowPlugin};
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt;

use crate::bevy_server::buffer::{BufferWriter, LOG_BUFFER};
use crate::bevy_server::resources::{CommandResponseSender, StdinReceiver};
use crate::bevy_server::systems::console::stdin::process_stdin_commands;
use crate::bevy_server::systems::headless::log_headless_bevy_status;
use crate::console;

pub fn buffered_fmt_layer(_: &mut App) -> Option<bevy::log::BoxedFmtLayer> {
    Some(Box::new(fmt::Layer::default().with_writer(|| {
        BufferWriter {
            buf: LOG_BUFFER.clone(),
        }
    })))
}

pub fn run_headless_bevy_server() {
    eprintln!("Starting server instance...");
    let (rx, response_tx) = console::start_console();

    let mut app = App::new();
    app.insert_resource(StdinReceiver(Arc::new(Mutex::new(rx))));
    app.insert_resource(CommandResponseSender(response_tx));

    app.add_plugins(
        bevy::prelude::DefaultPlugins
            .set(LogPlugin {
                filter: "info".to_string(),
                fmt_layer: buffered_fmt_layer,
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..Default::default()
            }),
    )
    .add_systems(Update, log_headless_bevy_status)
    .add_systems(Update, process_stdin_commands);

    app.run();
}
