use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    prelude::Res,
    transform::components::Transform,
};
use rand::{random, Rng};

use crate::{
    components::sun::Sun,
    events::spawn_sprite_event::{SpawnAnimatedSprite, SpawnSprite, SpawnSpriteEvent},
    resources::{constants::SPACE_TILE_SIZE, game_settings::GameSettings},
};

// TODO better spacing
// TODO better distribution across the map (maybe spawn zones)
pub fn spawn_suns(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    game_settings: Res<GameSettings>,
) {
    let mut rng = rand::thread_rng();
    let number_of_suns = game_settings.number_of_players;

    for _ in 0..number_of_suns {
        let sun = Sun::new(random());

        let x: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32)
                ..SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32),
        );
        let y: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32)
                ..SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32),
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
