use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    transform::components::Transform,
};
use rand::{random, Rng};

use crate::{
    components::sun::Sun,
    events::spawn_sprite_event::{SpawnAnimatedSprite, SpawnSprite, SpawnSpriteEvent},
    resources::constants::{NUMBER_OF_TILES, SPACE_TILE_SIZE},
};

pub fn spawn_suns(mut commands: Commands, mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>) {
    let mut rng = rand::thread_rng();
    let number_of_suns = rng.gen_range(3..6);

    for _ in 0..number_of_suns {
        let sun = Sun::new(random());

        let x: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 1) as f32)
                ..SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 1) as f32),
        );
        let y: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 1) as f32)
                ..SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 1) as f32),
        );

        let sun_transform = Transform {
            translation: Vec3::new(x, y, sun.size_component.z_index),
            ..Default::default()
        };

        spawn_sprite_event.send(SpawnSpriteEvent::spawn_animated_sprite(
            SpawnSprite {
                sprite_path: sun.sprite_path.to_string(),
                size: sun.size_component.size,
                transform: sun_transform,
                entity: commands.spawn(sun).insert(sun_transform).id(),
            },
            SpawnAnimatedSprite {
                sprite_tile_size: 200,
                frame_timing: 0.1,
                frame_count: 50,
            },
        ));
    }
}
