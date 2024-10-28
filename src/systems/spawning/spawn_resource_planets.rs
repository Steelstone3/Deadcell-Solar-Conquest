use crate::{
    components::map::planet::{PLANET_CLOSEST_DISTANCE_TO_SUN, Planet},
    events::{
        spawn_animated_sprite_event::{SpawnAnimatedSprite, SpawnAnimatedSpriteEvent},
        spawn_sprite_event::SpawnSprite,
    },
    queries::space_queries::SunQuery,
};
use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::Query,
};
use rand::{Rng, random};
use std::cmp;

pub fn spawn_resource_planets(
    mut commands: Commands,
    mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
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
            transform.translation.z = planet.size_component.z_index;

            spawn_animated_sprite_event.write(SpawnAnimatedSpriteEvent::spawn_animated_sprite(
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
