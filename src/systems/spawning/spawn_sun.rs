use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    transform::components::Transform,
};
use rand::random;

use crate::{components::sun::Sun, events::spawn_sprite_event::SpawnSpriteEvent};

pub fn spawn_sun(mut commands: Commands, mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>) {
    let sun = Sun::new(random());

    spawn_sprite_event.send(SpawnSpriteEvent {
        sprite_path: sun.sprite_path.to_string(),
        size: sun.size,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, sun.z_index),
            ..Default::default()
        },
        entity: commands.spawn(sun).id(),
    });
}
