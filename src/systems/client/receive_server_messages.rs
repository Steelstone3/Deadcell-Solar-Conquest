// use bevy::{
//     ecs::system::ResMut,
//     prelude::{Commands, EventWriter, Query, Transform},
//     utils::HashMap,
// };
// use bevy_renet::renet;
// use renet::RenetClient;

// use crate::{
//     components::{
//         faction::{space_facility::SerializableSpaceFacility, starship::SerializableStarship},
//         map::{planet::SerializablePlanet, space::SerializableSpace},
//         server::{
//             server_messages::ServerMessages,
//             server_object::{SerializableServerObject, ServerObject},
//         },
//         user_interface::selection::Selectable,
//     },
//     events::{
//         spawn_animated_sprite_event::{SpawnAnimatedSprite, SpawnAnimatedSpriteEvent},
//         spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
//     },
//     resources::lobby::Lobby,
//     server::channels::GameSyncChannels,
// };

// pub fn receive_server_messages(
//     mut client: ResMut<RenetClient>,
//     mut lobby: ResMut<Lobby>,
//     mut spawn_sprite_event_writer: EventWriter<SpawnSpriteEvent>,
//     mut spawn_animated_sprite_event_writer: EventWriter<SpawnAnimatedSpriteEvent>,
//     mut commands: Commands,
//     mut server_object_query: Query<(&mut Transform, &ServerObject)>,
// ) {
//     while let Some(message) = client.receive_message(GameSyncChannels::Messages) {
//         let server_message = bincode::deserialize(&message).unwrap();
//         match server_message {
//             ServerMessages::PlayerConnected { id } => {
//                 println!("Player {} connected.", id);
//                 lobby.players.push(id);
//             }
//             ServerMessages::PlayerDisconnected { id } => {
//                 println!("Player {} disconnected.", id);
//                 lobby.players.retain(|&x| x != id);
//             }
//         }
//     }

//     while let Some(message) = client.receive_message(GameSyncChannels::SpaceTiles) {
//         println!("Receiving space tiles");
//         let message: HashMap<u32, Vec<u8>> = bincode::deserialize(&message).unwrap();
//         for (_id, data) in message.iter() {
//             let space_tile: SerializableSpace = bincode::deserialize(data).unwrap();
//             let space = space_tile.space;
//             spawn_sprite_event_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
//                 sprite_path: space.sprite_path.to_string(),
//                 size: space.size_component.size,
//                 transform: space_tile.transform,
//                 entity: commands.spawn(space).id(),
//             }));
//         }
//     }

//     while let Some(message) = client.receive_message(GameSyncChannels::Planets) {
//         println!("Receiving planets");
//         let message: HashMap<u32, Vec<u8>> = bincode::deserialize(&message).unwrap();
//         for (id, data) in message.iter() {
//             println!("Spawning planet {:?}", id);
//             let planet: SerializablePlanet = bincode::deserialize(data).unwrap();
//             let planet_sprite = planet.planet;
//             spawn_animated_sprite_event_writer.write(SpawnAnimatedSprite::spawn_animated_sprite(
//                 SpawnSprite {
//                     sprite_path: planet_sprite.sprite_path.to_string(),
//                     size: planet_sprite.size_component.size,
//                     transform: planet.transform,
//                     entity: commands.spawn(planet_sprite).id(),
//                 },
//                 SpawnAnimatedSprite {
//                     sprite_tile_size: 100,
//                     frame_timing: 0.1,
//                     frame_count: 50,
//                 },
//             ));
//         }
//     }

//     while let Some(message) = client.receive_message(GameSyncChannels::SpaceFacilities) {
//         println!("Receiving space facilities");
//         let message: HashMap<u32, Vec<u8>> = bincode::deserialize(&message).unwrap();
//         for (id, data) in message.iter() {
//             println!("Spawning space facility {:?}", id);
//             let space_facility: SerializableSpaceFacility = bincode::deserialize(data).unwrap();
//             let space_facility_sprite = space_facility.space_facility;
//             spawn_sprite_event_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
//                 sprite_path: space_facility_sprite.sprite_path.to_string(),
//                 size: space_facility_sprite.size_component.size,
//                 transform: space_facility.transform,
//                 entity: commands
//                     .spawn(space_facility_sprite)
//                     .insert(Selectable)
//                     .id(),
//             }));
//         }
//     }

//     while let Some(message) = client.receive_message(GameSyncChannels::Starships) {
//         println!("Receiving starships");
//         let message: HashMap<u32, Vec<u8>> = bincode::deserialize(&message).unwrap();
//         for (id, data) in message.iter() {
//             println!("Spawning starship {:?}", id);
//             let starship: SerializableStarship = bincode::deserialize(data).unwrap();
//             let starship_sprite = starship.starship;
//             spawn_sprite_event_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
//                 sprite_path: starship_sprite
//                     .starship_sprite_bundle
//                     .starship_sprite
//                     .to_string(),
//                 size: starship_sprite.size_component.size,
//                 transform: starship.transform,
//                 entity: commands
//                     .spawn(starship_sprite)
//                     .insert(Selectable)
//                     .insert(starship.server_object)
//                     .id(),
//             }));
//         }
//     }

//     while let Some(message) = client.receive_message(GameSyncChannels::ServerObjects) {
//         let message: HashMap<u32, Vec<u8>> = bincode::deserialize(&message).unwrap();

//         for mut local_server_object in server_object_query.iter_mut() {
//             for (_id, data) in message.iter() {
//                 let remote_server_object: SerializableServerObject =
//                     bincode::deserialize(data).unwrap();
//                 if *local_server_object.1 == remote_server_object.server_object {
//                     *local_server_object.0 = remote_server_object.transform;
//                 }
//             }
//         }
//     }
// }

use bevy::{
    ecs::{
        event::EventWriter,
        system::{Commands, ResMut},
    },
    platform::collections::HashMap,
};
use bevy_renet::renet::RenetClient;
use bincode::{config, serde::decode_from_slice};

use crate::{
    components::{
        faction::space_facility::SerializableSpaceFacility,
        map::{planet::SerializablePlanet, space::SerializableSpace},
        server::server_messages::ServerMessages,
        user_interface::selection::Selectable,
    },
    events::{
        spawn_animated_sprite_event::{SpawnAnimatedSprite, SpawnAnimatedSpriteEvent},
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
    resources::lobby::Lobby,
    server::channels::GameSyncChannels,
};

pub fn receive_server_messages(
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut spawn_sprite_event_writer: EventWriter<SpawnSpriteEvent>,
    mut spawn_animated_sprite_event_writer: EventWriter<SpawnAnimatedSpriteEvent>,
    mut commands: Commands,
    // mut server_object_query: Query<(&mut Transform, &ServerObject)>,
) {
    while let Some(message) = client.receive_message(GameSyncChannels::Messages) {
        let (server_message, _): (ServerMessages, usize) =
            decode_from_slice(&message, config::standard()).unwrap();

        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);
                lobby.players.push(id);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
                lobby.players.retain(|&x| x != id);
            }
        }
    }

    while let Some(message) = client.receive_message(GameSyncChannels::SpaceTiles) {
        println!("Receiving space tiles");
        let (message, _): (HashMap<u32, Vec<u8>>, usize) =
            decode_from_slice(&message, config::standard()).unwrap();

        for (_id, data) in message.iter() {
            let (space_tile, _): (SerializableSpace, usize) =
                decode_from_slice(data, config::standard()).unwrap();

            let space = space_tile.space;
            spawn_sprite_event_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
                sprite_path: space.sprite_path.to_string(),
                size: space.size_component.size,
                transform: space_tile.transform,
                entity: commands.spawn(space).id(),
            }));
        }
    }

    while let Some(message) = client.receive_message(GameSyncChannels::Planets) {
        println!("Receiving planets");
        let (message, _): (HashMap<u32, Vec<u8>>, usize) =
            decode_from_slice(&message, config::standard()).unwrap_or_default();

        for (id, data) in message.iter() {
            println!("Spawning planet {:?}", id);
            let (planet, _): (SerializablePlanet, usize) =
                decode_from_slice(data, config::standard()).unwrap();

            let planet_sprite = planet.planet;
            spawn_animated_sprite_event_writer.write(
                SpawnAnimatedSpriteEvent::spawn_animated_sprite(
                    SpawnSprite {
                        sprite_path: planet_sprite.sprite_path.to_string(),
                        size: planet_sprite.size_component.size,
                        transform: planet.transform,
                        entity: commands.spawn(planet_sprite).id(),
                    },
                    SpawnAnimatedSprite {
                        sprite_tile_size: 100,
                        frame_timing: 0.1,
                        frame_count: 50,
                    },
                ),
            );
        }
    }

    while let Some(message) = client.receive_message(GameSyncChannels::SpaceFacilities) {
        println!("Receiving space facilities");
        let (message, _): (HashMap<u32, Vec<u8>>, usize) =
            decode_from_slice(&message, config::standard()).unwrap_or_default();

        for (id, data) in message.iter() {
            println!("Spawning space facility {:?}", id);
            let (space_facility, _): (SerializableSpaceFacility, usize) =
                decode_from_slice(data, config::standard()).unwrap();

            let space_facility_sprite = space_facility.space_facility;
            spawn_sprite_event_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
                sprite_path: space_facility_sprite.sprite_path.to_string(),
                size: space_facility_sprite.size_component.size,
                transform: space_facility.transform,
                entity: commands
                    .spawn(space_facility_sprite)
                    .insert(Selectable)
                    .id(),
            }));
        }
    }
}
