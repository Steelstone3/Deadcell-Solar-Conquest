use std::{net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, RenetServer,
};

//only allow client to connect with matching protocol id.
//Increment with each release
const PROTOCOL_ID: u64 = 0;

pub struct Server {}

impl Server {
    pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
        let public_addr = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind(public_addr).unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![public_addr],
            authentication: ServerAuthentication::Unsecure,
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        let server = RenetServer::new(ConnectionConfig::default());

        (server, transport)
    }
}