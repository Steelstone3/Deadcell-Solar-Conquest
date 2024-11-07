use bevy::prelude::{EventReader, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};

use crate::{components::server::server_messages::ServerMessages, resources::lobby::Lobby};

pub fn server_update(
    mut server_events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
                //TODO On Player Connect add to lobby list

                //Send connected message to all players
                for &player_id in lobby.players.iter() {
                    let message =
                        bincode::serialize(&ServerMessages::PlayerConnected { id: player_id })
                            .unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
                }

                lobby.players.push(*client_id);

                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                //on disconnect remove player from lobby
                lobby.players.retain(|&x| x != *client_id);

                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }

    /* for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_input);
            }
        }
    } */
}
