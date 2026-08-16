use bevy::prelude::Resource;
use std::sync::{Arc, Mutex};

type Rx = std::sync::mpsc::Receiver<String>;

#[derive(Resource)]
pub struct StdinReceiver(pub Arc<Mutex<Rx>>);
