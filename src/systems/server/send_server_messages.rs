use crate::{
    components::{
        faction::{space_facility::SpaceFacility, starship::Starship},
        map::{planet::Planet, space::Space, star::Star},
        server::{server_messages::ServerMessages, server_object::ServerObject},
    },
    resources::lobby::Lobby,
    server::channels::GameSyncChannels,
};
use bevy::{
    ecs::system::Query,
    prelude::{Entity, EventReader, ResMut},
    transform::components::Transform,
};
use bevy_renet::renet::{RenetServer, ServerEvent};
use bincode::{config, serde::encode_to_vec};

pub fn send_server_messages(
    mut server: ResMut<RenetServer>,
    space_tile_query: Query<(&Space, &Transform, Entity)>,
    planet_query: Query<(&Planet, &Transform, Entity)>,
    space_facility_query: Query<(&SpaceFacility, &Transform, Entity)>,
    starships_query: Query<(&Starship, &Transform, &ServerObject)>,
    star_query: Query<(&Star, &Transform, Entity)>,
    mut server_events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                lobby.players.push(*client_id);

                GameSyncChannels::Messages.send_message(
                    &mut server,
                    client_id,
                    &space_tile_query,
                    &starships_query,
                    &space_facility_query,
                    &planet_query,
                    &star_query,
                );
                GameSyncChannels::ServerObjects.send_message(
                    &mut server,
                    client_id,
                    &space_tile_query,
                    &starships_query,
                    &space_facility_query,
                    &planet_query,
                    &star_query,
                );
                GameSyncChannels::SpaceTiles.send_message(
                    &mut server,
                    client_id,
                    &space_tile_query,
                    &starships_query,
                    &space_facility_query,
                    &planet_query,
                    &star_query,
                );
                GameSyncChannels::Starships.send_message(
                    &mut server,
                    client_id,
                    &space_tile_query,
                    &starships_query,
                    &space_facility_query,
                    &planet_query,
                    &star_query,
                );
                GameSyncChannels::Planets.send_message(
                    &mut server,
                    client_id,
                    &space_tile_query,
                    &starships_query,
                    &space_facility_query,
                    &planet_query,
                    &star_query,
                );
                GameSyncChannels::Stars.send_message(
                    &mut server,
                    client_id,
                    &space_tile_query,
                    &starships_query,
                    &space_facility_query,
                    &planet_query,
                    &star_query,
                );
                GameSyncChannels::SpaceFacilities.send_message(
                    &mut server,
                    client_id,
                    &space_tile_query,
                    &starships_query,
                    &space_facility_query,
                    &planet_query,
                    &star_query,
                );
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
