use bevy::prelude::{App, Plugin};

use crate::events::{
    input_events::{MouseClickEvent, MouseRightClickEvent},
    spawn_sprite_event::SpawnSpriteEvent,
    user_interface_events::UserInterfaceEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpriteEvent>();
        app.add_event::<MouseClickEvent>();
        app.add_event::<MouseRightClickEvent>();
        app.add_event::<UserInterfaceEvent>();
    }
}
