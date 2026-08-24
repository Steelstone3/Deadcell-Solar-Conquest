use bevy_replicon_renet::{RenetServer, netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, renet::{ConnectionConfig, DefaultChannel}};
use std::{io::Error, net::UdpSocket, time::SystemTime};

// TODO move to connection configuration system
pub fn create_server_configuration() -> RenetServer {
    RenetServer::new(ConnectionConfig {
        client_channels_config: DefaultChannel::config(),
        server_channels_config: DefaultChannel::config(),
        ..Default::default()
    })
}

pub fn create_server_transport_configuration() -> Result<NetcodeServerTransport, Error> {
    let server_address = "127.0.0.1:5000";
    let server_address = match server_address.parse() {
        Ok(addr) => addr,
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
            format!("Failed to bind server socket: {}", e),
        )
    })?;
    let server_config = ServerConfig {
        current_time: match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(_) => std::time::Duration::from_millis(0),
        },
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_address],
        authentication: ServerAuthentication::Unsecure,
    };

    NetcodeServerTransport::new(server_config, socket)
}
