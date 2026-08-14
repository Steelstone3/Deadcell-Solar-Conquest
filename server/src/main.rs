use bevy::{
    MinimalPlugins,
    app::{App, Update},
    ecs::system::ResMut,
};
use bevy_renet::{
    RenetServer, RenetServerPlugin, netcode::NetcodeServerPlugin, renet::DefaultChannel,
};
use bevy_renet::{
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::ConnectionConfig,
};
use std::{io::Error, net::UdpSocket, time::SystemTime};

const SERVER_ADDRESS: &str = "127.0.0.1:5000";

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
fn main() {
    let transport = match create_server_transport_configuration() {
        Ok(transport) => transport,
        Err(_) => return,
    };
    let server = create_server_configuration();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);

    app.insert_resource(server);
    app.insert_resource(transport);

    app.add_systems(Update, send_server_message_system);

    app.run();
}

// TODO move to connection configuration system
fn send_server_message_system(mut server: ResMut<RenetServer>) {
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message".as_bytes());

    println!("I am a message being sent from the server to the client");
}

// TODO move to connection configuration system
fn create_server_configuration() -> RenetServer {
    RenetServer::new(ConnectionConfig {
        client_channels_config: DefaultChannel::config(),
        server_channels_config: DefaultChannel::config(),
        ..Default::default()
    })
}

// TODO move to connection configuration system
fn create_server_transport_configuration() -> Result<NetcodeServerTransport, Error> {
    let server_address = match SERVER_ADDRESS.parse() {
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
