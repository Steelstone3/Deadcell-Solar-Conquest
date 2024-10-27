use bevy::prelude::{App, Plugin};

use crate::events::{
    input_events::{MouseLeftClickEvent, MouseLeftClickModifierEvent, MouseRightClickEvent},
    selection_events::SelectionAreaEvent,
    spawn_sprite_event::SpawnSpriteEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpriteEvent>();
        app.add_event::<MouseLeftClickEvent>();
        app.add_event::<MouseLeftClickModifierEvent>();
        app.add_event::<MouseRightClickEvent>();
        app.add_event::<SelectionAreaEvent>();
    }
}
