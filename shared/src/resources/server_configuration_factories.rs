use bevy_renet::{
    RenetClient, RenetServer, netcode::{ClientAuthentication, NetcodeClientTransport, NetcodeError, NetcodeServerTransport, ServerAuthentication, ServerConfig}, renet::{ConnectionConfig, DefaultChannel},
};
use std::{io::Error, net::UdpSocket, time::SystemTime};

const SERVER_ADDRESS: &str = "127.0.0.1:5000";

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

// pub fn create_transport_configuration(socket: &str) -> Result<NetcodeServerTransport, Error> {
//     let server_address = match socket.parse() {
//         Ok(server_address) => server_address,
//         Err(_) => {
//             return Err(Error::new(
//                 std::io::ErrorKind::InvalidInput,
//                 "Invalid server address",
//             ));
//         }
//     };

//     let socket = UdpSocket::bind(server_address).map_err(|e| {
//         Error::new(
//             std::io::ErrorKind::AddrInUse,
//             format!("Failed to bind socket: {}", e),
//         )
//     })?;
//     let server_config = ServerConfig {
//         current_time: SystemTime::now()
//             .duration_since(SystemTime::UNIX_EPOCH)
//             .unwrap_or_default(),
//         max_clients: 64,
//         protocol_id: 0,
//         public_addresses: vec![server_address],
//         authentication: ServerAuthentication::Unsecure,
//     };

//     NetcodeServerTransport::new(server_config, socket)
//         .map_err(|e| Error::other(format!("Failed to create server transport: {}", e)))
// }

pub fn create_client_transport_configuration() -> Result<NetcodeClientTransport, NetcodeError> {
    let authentication = ClientAuthentication::Unsecure {
        server_addr: SERVER_ADDRESS.parse().unwrap(),
        client_id: 0,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    
    NetcodeClientTransport::new(current_time, authentication, socket)
}

pub fn create_server_transport_configuration() -> Result<NetcodeServerTransport, Error> {
    let server_addr = SERVER_ADDRESS.parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    NetcodeServerTransport::new(server_config, socket)
}
