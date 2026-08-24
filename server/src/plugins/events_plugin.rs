use bevy::app::Plugin;
use deadcell_solar_conquest_shared::events::publishers::{
    spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_server_entity_event::SpawnServerEntityEvent, spawn_sprite_event::SpawnSpriteEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<SpawnSpriteEvent>();
        app.add_message::<SpawnAnimatedSpriteEvent>();
        app.add_message::<SpawnServerEntityEvent>();
    }
}
