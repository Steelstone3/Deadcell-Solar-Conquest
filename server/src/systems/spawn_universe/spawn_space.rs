use bevy::{
    ecs::system::{Commands, Res},
    math::Vec3,
    transform::components::Transform,
};
use bevy_replicon::shared::replication::Replicated;
use deadcell_solar_conquest_shared::{
    components::space::Space,
    resources::{constants::SPACE_TILE_SIZE, game_settings::GameSettings},
};
use rand::random;

pub fn spawn_space(mut commands: Commands, game_settings: Res<GameSettings>) {
    let mut space = Space::new(random());

    for x in -game_settings.map_size * game_settings.number_of_players as i32
        ..game_settings.map_size * game_settings.number_of_players as i32
    {
        for y in -game_settings.map_size * game_settings.number_of_players as i32
            ..game_settings.map_size * game_settings.number_of_players as i32
        {
            let transform = Transform {
                translation: Vec3::new(
                    (x as f32 * SPACE_TILE_SIZE) + (SPACE_TILE_SIZE / 2.0),
                    (y as f32 * SPACE_TILE_SIZE) + (SPACE_TILE_SIZE / 2.0),
                    space.size_component.z_index,
                ),
                ..Default::default()
            };

            space.transform = transform;

            commands.spawn((space, space.transform, Replicated));
        }
    }
}

// TODO Client side
// Sprite::from_image(asset_server.load(space.sprite_path.to_string())),
