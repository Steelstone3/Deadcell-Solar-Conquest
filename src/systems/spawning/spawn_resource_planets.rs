use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::Commands,
    },
    math::Quat,
};
use rand::{random, Rng};

use crate::{
    components::planet::Planet,
    events::{
        spawn_animated_sprite_event::SpawnAnimatedSpriteEvent,
        spawn_planet_event::SpawnPlanetEvent, spawn_sprite_event::SpawnSpriteEvent,
    },
    resources::constants::PLANET_DISTANCE_FROM_SPACESTATION,
};

pub fn spawn_resource_planets(
    mut commands: Commands,
    mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
    mut spawn_planet_event_reader: EventReader<SpawnPlanetEvent>,
) {
    for spawn_planet_event in spawn_planet_event_reader.read() {
        let mut rng = rand::thread_rng();
        let angle: f32 = rng.gen_range(0.0..360.0);
        let planet = Planet::new(random());
        let mut transform = spawn_planet_event
            .space_station_transform
            .with_rotation(Quat::from_rotation_z(angle.to_radians()));

        transform.translation += transform.up() * PLANET_DISTANCE_FROM_SPACESTATION;

        spawn_animated_sprite_event.send(SpawnAnimatedSpriteEvent {
            sprite_tile_size: 100.0,
            frame_timing: 0.1,
            frame_count: 50,
            spawn_sprite_event: SpawnSpriteEvent {
                sprite_path: planet.sprite_path.to_string(),
                size: planet.size,
                transform,
                entity: commands.spawn(planet).id(),
            },
        });
    }
}
