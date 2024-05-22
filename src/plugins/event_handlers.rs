use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

use crate::events::event_handlers::{
    spawn_animated_sprite::spawn_animated_sprite, spawn_sprite::spawn_sprite,
};

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_sprite, spawn_animated_sprite));
    }
}
