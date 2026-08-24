use bevy::{MinimalPlugins, app::{App, Update}, ecs::system::ResMut};
use bevy_renet::{
    RenetServer, RenetServerPlugin, netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig}, renet::{ConnectionConfig, DefaultChannel},
};
use std::{io::Error, net::UdpSocket, time::SystemTime};

fn main() {
    let transport = match create_server_configuration() {
        Ok(transport) => transport,
        Err(_) => return,
    };
    let server = RenetServer::new(ConnectionConfig::default());

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.insert_resource(server);

    // Transport layer setup
    app.add_plugins(NetcodeServerPlugin);
    app.insert_resource(transport);
    
    app.add_systems(Update,send_message_system);
    app.add_systems(Update,receive_message_system);

    app.run();
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

fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    // Send a text message for all clients
    // The enum DefaultChannel describe the channels used by the default configuration
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
    println!("I am a message being sent from the server to the client");
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            println!("hello this is the bytes message being recieved");
            // Handle received message
        }
    }
}
