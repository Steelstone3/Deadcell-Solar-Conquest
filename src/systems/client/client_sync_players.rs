use bevy::ecs::system::ResMut;
use bevy_renet::renet;
use renet::{ClientId, DefaultChannel, RenetClient};

use crate::{components::server::server_messages::ServerMessages, resources::lobby::Lobby};

pub fn client_sync_players(mut client: ResMut<RenetClient>, mut lobby: ResMut<Lobby>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);
                //handle client side spawning here
                lobby.players.push(id);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
                lobby.players.retain(|&x| x != id);
            }
        }
    }

    /*    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let players: HashMap<u64, [f32; 3]> = bincode::deserialize(&message).unwrap();
        for (player_id, translation) in players.iter() {}
    } */
}
