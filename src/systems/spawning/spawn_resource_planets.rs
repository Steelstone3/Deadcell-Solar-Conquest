use crate::{
    components::planet::{Planet, PLANET_CLOSEST_DISTANCE_TO_SUN},
    events::spawn_sprite_event::{SpawnAnimatedSprite, SpawnSprite, SpawnSpriteEvent},
    queries::space_queries::SunQuery,
};
use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::Query,
};
use rand::{random, Rng};
use std::cmp;

pub fn spawn_resource_planets(
    mut commands: Commands,
    mut spawn_animated_sprite_event: EventWriter<SpawnSpriteEvent>,
    sun_queries: Query<SunQuery>,
) {
    for sun_query in sun_queries.iter() {
        let mut rng = rand::thread_rng();

        let number_of_planets = rng.gen_range(3..6);

        for _ in 0..number_of_planets {
            let planet = Planet::new(random());
            let angle: f32 = rng.gen_range(0.0..360.0);
            let mut transform = sun_query
                .transform
                .with_rotation(Quat::from_rotation_z(angle.to_radians()));

            transform.translation +=
                transform.up() * PLANET_CLOSEST_DISTANCE_TO_SUN * rng.gen_range(1.0..3.0)
                    + (cmp::max(
                        planet.size_component.size.x as u32,
                        planet.size_component.size.y as u32,
                    )) as f32;

            spawn_animated_sprite_event.send(SpawnSpriteEvent::spawn_animated_sprite(
                SpawnSprite {
                    sprite_path: planet.sprite_path.to_string(),
                    size: planet.size_component.size,
                    transform,
                    entity: commands.spawn(planet).id(),
                },
                SpawnAnimatedSprite {
                    sprite_tile_size: 100,
                    frame_timing: 0.1,
                    frame_count: 50,
                },
            ));
        }
    }
}
