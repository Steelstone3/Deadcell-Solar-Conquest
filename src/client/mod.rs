use std::{net::UdpSocket, time::SystemTime};

use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet::{ConnectionConfig, RenetClient},
};

use crate::server::game_server::Server;

pub struct Client {}

impl Client {
    pub fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
        #[allow(clippy::unwrap_used)]
        let server_addr = "127.0.0.1:5000".parse().unwrap();
        #[allow(clippy::unwrap_used)]
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: Server::get_protocol_id(),
            server_addr,
            user_data: None,
        };

        #[allow(clippy::unwrap_used)]
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        let client = RenetClient::new(ConnectionConfig::default());

        (client, transport)
    }
}
