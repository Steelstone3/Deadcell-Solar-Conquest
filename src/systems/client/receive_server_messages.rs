use bevy::{
    ecs::system::ResMut,
    prelude::{Commands, EventWriter},
    utils::HashMap,
};
use bevy_renet::renet;
use renet::RenetClient;

use crate::{
    components::{map::space::SerializableSpace, server::server_messages::ServerMessages},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    resources::lobby::Lobby,
    server::channels::GameSyncChannels,
};

pub fn receive_server_messages(
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut spawn_sprite_event_writer: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
) {
    while let Some(message) = client.receive_message(GameSyncChannels::Messages) {
        let server_message = bincode::deserialize(&message).unwrap();
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

        let message: HashMap<u64, Vec<u8>> = bincode::deserialize(&message).unwrap();
        for (_id, data) in message.iter() {
            let space_tile: SerializableSpace = bincode::deserialize(&data).unwrap();
            let space = space_tile.space;
            spawn_sprite_event_writer.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
                sprite_path: space.sprite_path.to_string(),
                size: space.size_component.size,
                transform: space_tile.transform,
                entity: commands.spawn(space).id(),
            }));
        }
    }

    while let Some(_message) = client.receive_message(GameSyncChannels::Planets) {
        println!("Receiving planets");
        //TODO receive planets
    }

    while let Some(_message) = client.receive_message(GameSyncChannels::SpaceFacilities) {
        println!("Receiving space facilities");
        //TODO receive facilities
    }

    while let Some(_message) = client.receive_message(GameSyncChannels::Starships) {
        println!("Receiving starships");
        //TODO receive starships
    }
}
