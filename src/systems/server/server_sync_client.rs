use bevy::{
    ecs::system::Query,
    prelude::{Entity, EventReader, ResMut},
    transform::components::Transform,
    utils::hashbrown::HashMap,
};
use bevy_renet::renet::{DefaultChannel, RenetServer};

use crate::{
    components::map::space::{SerializableSpace, Space},
    events::player_connected::PlayerConnectedEvent,
};

pub fn server_sync_client(
    mut server: ResMut<RenetServer>,
    space_tile_query: Query<(&Space, &Transform, Entity)>,
    mut player_connected_event_reader: EventReader<PlayerConnectedEvent>,
) {
    for connected_player_event in player_connected_event_reader.read() {
        println!("Syncing world with connected player");
        let mut space_tiles: HashMap<u64, Vec<u8>> = HashMap::new();
        for (space, transform, entity) in space_tile_query.iter() {
            let space_tile =
                bincode::serialize(&SerializableSpace::new(*space, *transform)).unwrap();
            space_tiles.insert(entity.to_bits(), space_tile.to_owned());
        }

        let message = bincode::serialize(&space_tiles).unwrap();
        server.send_message(
            connected_player_event.client_id,
            DefaultChannel::ReliableUnordered,
            message,
        );
    }
}
