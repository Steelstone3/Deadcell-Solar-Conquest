use bevy::{
    MinimalPlugins,
    app::{App, Update},
    ecs::system::ResMut,
};
use bevy_renet::{
    RenetServer, RenetServerPlugin, netcode::NetcodeServerPlugin, renet::DefaultChannel,
};
use deadcell_solar_conquest_shared::resources::server_configuration_factories::{
    create_server_configuration, create_transport_configuration,
};

fn main() {
    let transport = match create_transport_configuration() {
        Ok(transport) => transport,
        Err(_) => return,
    };
    let server = create_server_configuration();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.insert_resource(server);

    // Transport layer setup
    app.add_plugins(NetcodeServerPlugin);
    app.insert_resource(transport);

    app.add_systems(Update, send_server_message_system);

    app.run();
}

// fn send_message_system(mut server: ResMut<RenetServer>, time: Res<Time>) {
//     // 1. Advance the internal renet clock so messages process
//     server.update(time.delta());

//     // 2. Convert string payload to bytes (Renet expects Bytes or &[u8])
//     server.broadcast_message(
//         DefaultChannel::ReliableOrdered,
//         "server message".as_bytes()
//     );

//     println!("I am a message being sent from the server to the client");
// }

fn send_server_message_system(mut server: ResMut<RenetServer>) {
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message".as_bytes());

    println!("I am a message being sent from the server to the client");
}

// TODO server side
// fn receive_client_message_system(mut server: ResMut<RenetServer>) {
//     // Iterate over all connected clients to process their incoming messages
//     for client_id in server.clients_id() {
//         while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
//         {
//             match String::from_utf8(message.to_vec()) {
//                 Ok(text) => println!("Received from client {}: {}", client_id, text),
//                 Err(e) => eprintln!("Failed to parse message bytes from {}: {}", client_id, e),
//             }
//         }
//     }
// }
