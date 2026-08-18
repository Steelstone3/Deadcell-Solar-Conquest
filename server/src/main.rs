use bevy::app::App;
use bevy_renet::{
    RenetServer, RenetServerPlugin,
    netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::ConnectionConfig,
};
use std::{io::Error, net::UdpSocket, time::SystemTime};

fn main() {
    let transport = match create_server_configuration() {
        Ok(transport) => transport,
        Err(_) => return,
    };
    let server = RenetServer::new(ConnectionConfig::default());

    let mut app = App::new();
    app.add_plugins(RenetServerPlugin);
    app.insert_resource(server);

    // Transport layer setup
    app.add_plugins(NetcodeServerPlugin);
    app.insert_resource(transport);
}

fn create_server_configuration() -> Result<NetcodeServerTransport, Error> {
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
