use bevy::prelude::Resource;
use crossbeam::channel::Sender as CrossbeamSender;

#[derive(Resource)]
pub struct CommandResponseSender(pub CrossbeamSender<String>);
