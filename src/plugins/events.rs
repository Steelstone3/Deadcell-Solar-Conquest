use bevy::prelude::{App, Plugin};

use crate::events::{
    mouse_click_event::MouseClickEvent, mouse_right_click_event::MouseRightClickEvent,
    spawn_sprite_event::SpawnSpriteEvent, user_interface_event::UserInterfaceEvent,
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
