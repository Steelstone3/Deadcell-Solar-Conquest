use bevy::prelude::{App, Plugin};

use crate::events::{
    input_events::{MouseLeftClickEvent, MouseLeftClickModifierEvent, MouseRightClickEvent},
    player_connected::PlayerConnectedEvent,
    selection_events::SelectionAreaEvent,
    spawn_animated_sprite_event::SpawnAnimatedSpriteEvent,
    spawn_sprite_event::SpawnSpriteEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnSpriteEvent>();
        app.add_message::<SpawnAnimatedSpriteEvent>();
        app.add_message::<MouseLeftClickEvent>();
        app.add_message::<MouseLeftClickModifierEvent>();
        app.add_message::<MouseRightClickEvent>();
        app.add_message::<SelectionAreaEvent>();
        app.add_message::<PlayerConnectedEvent>();
    }
}
