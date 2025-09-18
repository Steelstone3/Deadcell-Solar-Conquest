use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    prelude::Res,
    transform::components::Transform,
};
use rand::{Rng, random};

use crate::{
    components::map::star::Star,
    events::{
        spawn_animated_sprite_event::{SpawnAnimatedSprite, SpawnAnimatedSpriteEvent},
        spawn_sprite_event::SpawnSprite,
    },
    resources::{constants::SPACE_TILE_SIZE, game_settings::GameSettings},
};

// TODO better spacing
// TODO better distribution across the map (maybe spawn zones)
pub fn spawn_stars(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
    game_settings: Res<GameSettings>,
) {
    let mut rng = rand::thread_rng();
    let number_of_stars = game_settings.number_of_players;

    for _ in 0..number_of_stars {
        let star = Star::new(random());

        let x: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32)
                ..SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32),
        );
        let y: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32)
                ..SPACE_TILE_SIZE * ((game_settings.map_size - 1) as f32),
        );

        let star_transform = Transform {
            translation: Vec3::new(x, y, star.size_component.z_index),
            ..Default::default()
        };

        spawn_sprite_event.write(SpawnAnimatedSpriteEvent::spawn_animated_sprite(
            SpawnSprite {
                sprite_path: star.sprite_path.to_string(),
                size: star.size_component.size,
                transform: star_transform,
                entity: commands.spawn(star).insert(star_transform).id(),
            },
            SpawnAnimatedSprite {
                sprite_tile_size: 200,
                frame_timing: 0.1,
                frame_count: 50,
            },
        ));
    }
}
