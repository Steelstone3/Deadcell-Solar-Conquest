use std::time::Duration;

use bevy::{
    ecs::{entity::Entity, system::Query},
    platform::collections::HashMap,
    transform::components::Transform,
};
use bevy_renet::renet::{ChannelConfig, RenetServer, SendType};
use bincode::{config, serde::encode_to_vec};

use crate::components::{
    faction::{
        space_facility::{SerializableSpaceFacility, SpaceFacility},
        starship::{SerializableStarship, Starship},
    },
    map::{
        planet::{Planet, SerializablePlanet},
        space::{SerializableSpace, Space},
        star::{SerializableStar, Star},
    },
    server::{server_messages::ServerMessages, server_object::ServerObject},
};

pub enum GameSyncChannels {
    Messages,
    ServerObjects,
    SpaceTiles,
    Starships,
    SpaceFacilities,
    Planets,
    Stars,
}

impl From<GameSyncChannels> for u8 {
    fn from(channel: GameSyncChannels) -> Self {
        match channel {
            GameSyncChannels::Messages => 0,
            GameSyncChannels::ServerObjects => 1,
            GameSyncChannels::SpaceTiles => 2,
            GameSyncChannels::Starships => 3,
            GameSyncChannels::SpaceFacilities => 4,
            GameSyncChannels::Planets => 5,
            GameSyncChannels::Stars => 6,
        }
    }
}

impl GameSyncChannels {
    pub fn config() -> Vec<ChannelConfig> {
        vec![
            GameSyncChannels::channel_config_factory(0),
            GameSyncChannels::channel_config_factory(1),
            GameSyncChannels::channel_config_factory(2),
            GameSyncChannels::channel_config_factory(3),
            GameSyncChannels::channel_config_factory(4),
            GameSyncChannels::channel_config_factory(5),
            GameSyncChannels::channel_config_factory(6),
        ]
    }

    pub fn send_message(
        &self,
        server: &mut RenetServer,
        client_id: &u64,
        space_tile_query: &Query<(&Space, &Transform, Entity)>,
        starships_query: &Query<(&Starship, &Transform, &ServerObject)>,
        space_facility_query: &Query<(&SpaceFacility, &Transform, Entity)>,
        planet_query: &Query<(&Planet, &Transform, Entity)>,
        star_query: &Query<(&Star, &Transform, Entity)>,
    ) {
        let mut map: HashMap<u32, Vec<u8>> = HashMap::new();

        match self {
            GameSyncChannels::Messages => {
                // Send connected message
                let message = encode_to_vec(
                    ServerMessages::PlayerConnected { id: *client_id },
                    config::standard(),
                )
                .unwrap_or_default();
                server.broadcast_message(GameSyncChannels::Messages, message);
            }
            GameSyncChannels::ServerObjects => {}
            GameSyncChannels::SpaceTiles => {
                // Sync space tiles
                for (space, transform, entity) in space_tile_query.iter() {
                    let space_tile = encode_to_vec(
                        SerializableSpace::new(*space, *transform),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    map.insert(entity.index(), space_tile);
                }
                println!("Syncing space tiles with connected player");
                let message = encode_to_vec(&map, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::SpaceTiles, message);
            }
            GameSyncChannels::Starships => {
                // Sync starships
                for (starship, transform, server_object) in starships_query.iter() {
                    let starship = encode_to_vec(
                        SerializableStarship::new(*starship, *transform, *server_object),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    map.insert(server_object.id, starship);
                }
                println!("Syncing starships with connected player");
                let message = encode_to_vec(&map, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::Starships, message);
            }
            GameSyncChannels::SpaceFacilities => {
                // Sync space facilities
                for (space_facility, transform, entity) in space_facility_query.iter() {
                    let space_facility = encode_to_vec(
                        SerializableSpaceFacility::new(*space_facility, *transform),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    map.insert(entity.index(), space_facility);
                }
                println!("Syncing space facilities with connected player");
                let message = encode_to_vec(&map, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::SpaceFacilities, message);
            }
            GameSyncChannels::Planets => {
                // Sync planets
                for (planet, transform, entity) in planet_query.iter() {
                    let planet = encode_to_vec(
                        SerializablePlanet::new(*planet, *transform),
                        config::standard(),
                    )
                    .unwrap_or_default();
                    map.insert(entity.index(), planet);
                }
                println!("Syncing planets with connected player");
                let message = encode_to_vec(&map, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::Planets, message);
            }
            GameSyncChannels::Stars => {
                // Sync stars
                for (star, transform, entity) in star_query.iter() {
                    let star =
                        encode_to_vec(SerializableStar::new(*star, *transform), config::standard())
                            .unwrap_or_default();
                    map.insert(entity.index(), star);
                }
                println!("Syncing planets with connected player");
                let message = encode_to_vec(&map, config::standard()).unwrap_or_default();
                server.send_message(*client_id, GameSyncChannels::Stars, message);
            }
        }
    }

    fn channel_config_factory(channel_id: u8) -> ChannelConfig {
        ChannelConfig {
            channel_id,
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_millis(300),
            },
        }
    }
}
