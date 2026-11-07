use crate::events::spawn_sprite_event::{self, SpawnSpriteEvent, spawn_sprite_event_handler};
use bevy::{app::Update, prelude::{App, Plugin}};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnSpriteEvent>();
        app.add_systems(Update, spawn_sprite_event_handler);
        // app.add_systems(Update, spawn_animated_sprite_event_handler);
        // app.add_message::<SpawnAnimatedSpriteEvent>();
        // app.add_message::<MouseLeftClickEvent>();
        // app.add_message::<MouseLeftClickModifierEvent>();
        // app.add_message::<MouseRightClickEvent>();
        // app.add_message::<SelectionAreaEvent>();
    }
}
