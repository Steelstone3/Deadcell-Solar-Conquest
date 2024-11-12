use std::{net::UdpSocket, time::SystemTime};

use bevy::ecs::system::Commands;
use bevy_renet::renet;
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    ConnectionConfig, RenetClient,
};

use crate::server::{channels::GameSyncChannels, server::Server};

pub fn connect_to_server(mut commands: Commands) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: Server::get_protocol_id(),
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig {
        available_bytes_per_tick: 60_000,
        server_channels_config: GameSyncChannels::config(),
        client_channels_config: GameSyncChannels::config(),
    });

    commands.insert_resource(transport);
    commands.insert_resource(client);
}
