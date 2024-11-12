use bevy::{
    ecs::system::Query,
    prelude::{Entity, EventReader, ResMut},
    transform::components::Transform,
    utils::hashbrown::HashMap,
};
use bevy_renet::renet::{RenetServer, ServerEvent};

use crate::{
    components::{
        map::space::{SerializableSpace, Space},
        server::server_messages::ServerMessages,
    },
    resources::lobby::Lobby,
    server::channels::GameSyncChannels,
};

pub fn send_server_messages(
    mut server: ResMut<RenetServer>,
    space_tile_query: Query<(&Space, &Transform, Entity)>,
    mut server_events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                lobby.players.push(*client_id);

                //Send connected message to all players
                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(GameSyncChannels::Messages, message);

                let mut space_tiles: HashMap<u64, Vec<u8>> = HashMap::new();
                for (space, transform, entity) in space_tile_query.iter() {
                    let space_tile =
                        bincode::serialize(&SerializableSpace::new(*space, *transform)).unwrap();
                    space_tiles.insert(entity.to_bits(), space_tile.to_owned());
                }

                println!("Syncing space tiles with connected player");
                let message = bincode::serialize(&space_tiles).unwrap();
                server.send_message(*client_id, GameSyncChannels::SpaceTiles, message);

                println!("Syncing planets with connected player");
                //TODO Sync planets

                println!("Syncing space facilities with connected player");
                //TODO Sync planets

                println!("Syncing starships with connected player");
                //TODO Sync planets
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                //on disconnect remove player from lobby
                lobby.players.retain(|&x| x != *client_id);

                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(GameSyncChannels::Messages, message);

                //TODO Despawn players stuff on disconnect
            }
        }
    }
}
