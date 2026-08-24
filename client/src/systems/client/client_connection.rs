use std::{net::UdpSocket, time::SystemTime};

use bevy_replicon_renet::{RenetClient, netcode::{ClientAuthentication, NetcodeClientTransport, NetcodeError}, renet::{ConnectionConfig, DefaultChannel}};

pub fn create_client_configuration() -> RenetClient {
    RenetClient::new(ConnectionConfig {
        client_channels_config: DefaultChannel::config(),
        server_channels_config: DefaultChannel::config(),
        ..Default::default()
    })
}

pub fn create_client_transport_configuration() -> Result<NetcodeClientTransport, NetcodeError> {
    let server_address = "127.0.0.1:5000";
    let client_address = "127.0.0.1:0";
    let authentication = ClientAuthentication::Unsecure {
        server_addr: match server_address.parse() {
            Ok(addr) => addr,
            Err(_) => return Err(NetcodeError::ClientNotConnected),
        },
        client_id: 0,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind(client_address)?;
    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => std::time::Duration::from_millis(0),
    };

    NetcodeClientTransport::new(current_time, authentication, socket)
}
