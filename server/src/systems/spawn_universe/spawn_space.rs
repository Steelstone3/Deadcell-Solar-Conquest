use bevy::{
    ecs::{
        message::MessageWriter,
        system::{Commands, Res},
    },
    math::Vec3,
    transform::components::Transform,
};
use deadcell_solar_conquest_shared::{
    components::space::Space,
    events::publishers::{
        spawn_entity::SpawnEntity, spawn_server_entity_event::SpawnServerEntityEvent,
    },
    resources::{constants::SPACE_TILE_SIZE, game_settings::GameSettings},
};
use rand::random;

pub fn spawn_space(
    mut commands: Commands,
    mut spawn_server_entity_event: MessageWriter<SpawnServerEntityEvent>,
    game_settings: Res<GameSettings>,
) {
    let space = Space::new(random());

    for x in -game_settings.map_size * game_settings.number_of_players as i32
        ..game_settings.map_size * game_settings.number_of_players as i32
    {
        for y in -game_settings.map_size * game_settings.number_of_players as i32
            ..game_settings.map_size * game_settings.number_of_players as i32
        {
            spawn_server_entity_event.write(SpawnServerEntityEvent::spawn_sprite(SpawnEntity {
                sprite_path: space.sprite_path.to_string(),
                size: space.size_component.size,
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 * SPACE_TILE_SIZE) + (SPACE_TILE_SIZE / 2.0),
                        (y as f32 * SPACE_TILE_SIZE) + (SPACE_TILE_SIZE / 2.0),
                        space.size_component.z_index,
                    ),
                    ..Default::default()
                },
                entity: commands.spawn(space).id(),
            }));
        }
    }
}

// fn spawn_player_and_notify_client(
//     mut commands: Commands,
//     mut server: ResMut<RenetServer>,
// ) {
//     // 1. Target a specific client ID (e.g., ClientId::from_raw(1))
//     // Or iterate through connected clients using server.clients_id()
//     let client_id = 1;

//     // 2. Spawn an entity on the server
//     let server_entity = commands.spawn((
//         SpatialBundle::default(), // Or TransformBundle, etc.
//         // Add your server-side components here
//     )).id();

//     // 3. Prepare your network payload (typically serialized using bincode, serde, etc.)
//     let message_payload = format!("Spawned player entity index: {:?}", server_entity);

//     // 4. Send the message over a specified channel (e.g., ReliableOrdered)
//     server.send_message(
//         client_id,
//         DefaultChannel::ReliableOrdered,
//         message_payload.into_bytes(),
//     );
// }
