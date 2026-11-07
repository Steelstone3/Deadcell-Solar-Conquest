use bevy::{
    ecs::system::{Commands, Res, ResMut},
    math::Vec3,
    transform::components::Transform,
};
use bevy_replicon::shared::replication::Replicated;
use bevy_replicon_renet::{RenetServer, renet::DefaultChannel};
use deadcell_solar_conquest_shared::{
    components::space::Space,
    messages::server_messages::ServerMessages,
    resources::{constants::SPACE_TILE_SIZE, game_settings::GameSettings},
};
use rand::random;

pub fn spawn_space_on_client_connected(
    mut commands: Commands,
    game_settings: &Res<GameSettings>,
    server: &mut ResMut<RenetServer>,
) {
    println!("Client entity connected! Generating world...",);

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

            // send spawn space server message
            let payload = bincode_next::serde::encode_to_vec(
                &ServerMessages::SpawnSpace { space },
                bincode_next::config::standard(),
            )
            .unwrap_or_default();

            // todo work out how to spawn space for specific client ids
            server.broadcast_message(DefaultChannel::ReliableOrdered, payload);
        }
    }
}

// TODO Client side
// Sprite::from_image(asset_server.load(space.sprite_path.to_string())),
