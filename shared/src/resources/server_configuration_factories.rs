use bevy_renet::{
    RenetClient, RenetServer,
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, DefaultChannel},
};
use std::{io::Error, net::UdpSocket, time::SystemTime};

pub fn create_server_configuration() -> RenetServer {
    let server = RenetServer::new(ConnectionConfig {
        client_channels_config: DefaultChannel::config(),
        server_channels_config: DefaultChannel::config(),
        ..Default::default()
    });
    server
}

pub fn create_client_configuration() -> RenetClient {
    RenetClient::new(ConnectionConfig {
        client_channels_config: DefaultChannel::config(),
        server_channels_config: DefaultChannel::config(),
        ..Default::default()
    })
}

pub fn create_transport_configuration() -> Result<NetcodeServerTransport, Error> {
    let server_address = match "127.0.0.1:5000".parse() {
        Ok(server_address) => server_address,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid server address",
            ));
        }
    };

    let socket = UdpSocket::bind(server_address).map_err(|e| {
        Error::new(
            std::io::ErrorKind::AddrInUse,
            format!("Failed to bind socket: {}", e),
        )
    })?;
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_address],
        authentication: ServerAuthentication::Unsecure,
    };

    NetcodeServerTransport::new(server_config, socket)
        .map_err(|e| Error::other(format!("Failed to create server transport: {}", e)))
}
