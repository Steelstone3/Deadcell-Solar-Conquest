// use bevy::{
//     ecs::system::Query,
//     prelude::{Entity, EventReader, ResMut},
//     transform::components::Transform,
//     utils::hashbrown::HashMap,
// };
// use bevy_renet::renet::{RenetServer, ServerEvent};

// use crate::{
//     components::{
//         faction::{
//             space_facility::{SerializableSpaceFacility, SpaceFacility},
//             starship::{SerializableStarship, Starship},
//         },
//         map::{
//             planet::{Planet, SerializablePlanet},
//             space::{SerializableSpace, Space},
//         },
//         server::{server_messages::ServerMessages, server_object::ServerObject},
//     },
//     resources::lobby::Lobby,
//     server::channels::GameSyncChannels,
// };

// pub fn send_server_messages(
//     mut server: ResMut<RenetServer>,
//     space_tile_query: Query<(&Space, &Transform, Entity)>,
//     planet_query: Query<(&Planet, &Transform, Entity)>,
//     space_facility_query: Query<(&SpaceFacility, &Transform, Entity)>,
//     starships_query: Query<(&Starship, &Transform, &ServerObject)>,
//     mut server_events: EventReader<ServerEvent>,
//     mut lobby: ResMut<Lobby>,
// ) {
//     for event in server_events.read() {
//         match event {
//             ServerEvent::ClientConnected { client_id } => {
//                 println!("Player {} connected.", client_id);

//                 lobby.players.push(*client_id);

//                 //Send connected message to all players
//                 let message =
//                     bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
//                         .unwrap();
//                 server.broadcast_message(GameSyncChannels::Messages, message);

//                 let mut space_tiles: HashMap<u32, Vec<u8>> = HashMap::new();
//                 for (space, transform, entity) in space_tile_query.iter() {
//                     let space_tile =
//                         bincode::serialize(&SerializableSpace::new(*space, *transform)).unwrap();
//                     space_tiles.insert(entity.index(), space_tile.to_owned());
//                 }
//                 println!("Syncing space tiles with connected player");
//                 let message = bincode::serialize(&space_tiles).unwrap();
//                 server.send_message(*client_id, GameSyncChannels::SpaceTiles, message);

//                 println!("Syncing planets with connected player");
//                 let mut planets: HashMap<u32, Vec<u8>> = HashMap::new();
//                 for (planet, transform, entity) in planet_query.iter() {
//                     let planet =
//                         bincode::serialize(&SerializablePlanet::new(*planet, *transform)).unwrap();
//                     planets.insert(entity.index(), planet.to_owned());
//                 }
//                 let message = bincode::serialize(&planets).unwrap();
//                 server.send_message(*client_id, GameSyncChannels::Planets, message);

//                 println!("Syncing space facilities with connected player");
//                 let mut space_facilities: HashMap<u32, Vec<u8>> = HashMap::new();
//                 for (space_facility, transform, entity) in space_facility_query.iter() {
//                     let space_facility = bincode::serialize(&SerializableSpaceFacility::new(
//                         *space_facility,
//                         *transform,
//                     ))
//                     .unwrap();
//                     space_facilities.insert(entity.index(), space_facility.to_owned());
//                 }
//                 let message = bincode::serialize(&space_facilities).unwrap();
//                 server.send_message(*client_id, GameSyncChannels::SpaceFacilities, message);

//                 println!("Syncing starships with connected player");
//                 let mut starships: HashMap<u32, Vec<u8>> = HashMap::new();
//                 for (starship, transform, server_object) in starships_query.iter() {
//                     let starship = bincode::serialize(&SerializableStarship::new(
//                         *starship,
//                         *transform,
//                         *server_object,
//                     ))
//                     .unwrap();
//                     starships.insert(server_object.id, starship.to_owned());
//                 }
//                 let message = bincode::serialize(&starships).unwrap();
//                 server.send_message(*client_id, GameSyncChannels::Starships, message);
//             }
//             ServerEvent::ClientDisconnected { client_id, reason } => {
//                 println!("Player {} disconnected: {}", client_id, reason);
//                 //on disconnect remove player from lobby
//                 lobby.players.retain(|&x| x != *client_id);

//                 let message =
//                     bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
//                         .unwrap();
//                 server.broadcast_message(GameSyncChannels::Messages, message);

//                 //TODO Despawn players stuff on disconnect
//             }
//         }
//     }
// }

use bevy::{
    ecs::system::Query,
    platform::collections::HashMap,
    prelude::{Entity, EventReader, ResMut},
    transform::components::Transform,
};
use bevy_renet::renet::{RenetServer, ServerEvent};

use bincode::{config, serde::encode_to_vec};

use crate::{
    components::{
        faction::{
            space_facility::{SerializableSpaceFacility, SpaceFacility},
            starship::{SerializableStarship, Starship},
        },
        map::{
            planet::{Planet, SerializablePlanet},
            space::{SerializableSpace, Space},
        },
        server::{server_messages::ServerMessages, server_object::ServerObject},
    },
    resources::lobby::Lobby,
    server::channels::GameSyncChannels,
};

pub fn send_server_messages(
    mut server: ResMut<RenetServer>,
    space_tile_query: Query<(&Space, &Transform, Entity)>,
    planet_query: Query<(&Planet, &Transform, Entity)>,
    space_facility_query: Query<(&SpaceFacility, &Transform, Entity)>,
    starships_query: Query<(&Starship, &Transform, &ServerObject)>,
    mut server_events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                lobby.players.push(*client_id);

                // Send connected message
                let message = encode_to_vec(
                    ServerMessages::PlayerConnected { id: *client_id },
                    config::standard(),
                )
                .unwrap_or_default();
                server.broadcast_message(GameSyncChannels::Messages, message);

                // Sync space tiles
                let mut space_tiles: HashMap<u32, Vec<u8>> = HashMap::new();
                for (space, transform, entity) in space_tile_query.iter() {
                    let space_tile = encode_to_vec(
                        SerializableSpace::new(*space, *transform),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    space_tiles.insert(entity.index(), space_tile);
                }
                println!("Syncing space tiles with connected player");
                let message = encode_to_vec(&space_tiles, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::SpaceTiles, message);

                // Sync planets
                println!("Syncing planets with connected player");
                let mut planets: HashMap<u32, Vec<u8>> = HashMap::new();
                for (planet, transform, entity) in planet_query.iter() {
                    let planet = encode_to_vec(
                        SerializablePlanet::new(*planet, *transform),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    planets.insert(entity.index(), planet);
                }
                let message = encode_to_vec(&planets, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::Planets, message);

                // Sync space facilities
                println!("Syncing space facilities with connected player");
                let mut space_facilities: HashMap<u32, Vec<u8>> = HashMap::new();
                for (space_facility, transform, entity) in space_facility_query.iter() {
                    let space_facility = encode_to_vec(
                        SerializableSpaceFacility::new(*space_facility, *transform),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    space_facilities.insert(entity.index(), space_facility);
                }
                let message =
                    encode_to_vec(&space_facilities, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::SpaceFacilities, message);

                // Sync starships
                println!("Syncing starships with connected player");
                let mut starships: HashMap<u32, Vec<u8>> = HashMap::new();
                for (starship, transform, server_object) in starships_query.iter() {
                    let starship = encode_to_vec(
                        SerializableStarship::new(*starship, *transform, *server_object),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    starships.insert(server_object.id, starship);
                }
                let message = encode_to_vec(&starships, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::Starships, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                lobby.players.retain(|&x| x != *client_id);

                let message = encode_to_vec(
                    &ServerMessages::PlayerDisconnected { id: *client_id },
                    config::standard(),
                )
                .unwrap_or_default();
                server.broadcast_message(GameSyncChannels::Messages, message);

                // TODO: Despawn players' stuff on disconnect
            }
        }
    }
}
