use bevy::prelude::{App, Plugin};

use crate::events::{
    mouse_click_event::MouseClickEvent, spawn_planet_event::SpawnPlanetEvent,
    spawn_sprite_event::SpawnSpriteEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpriteEvent>();
        app.add_event::<SpawnPlanetEvent>();
        app.add_event::<MouseClickEvent>();
    }
}
