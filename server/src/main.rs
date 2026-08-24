use bevy::{
    MinimalPlugins,
    app::{App, Update},
    ecs::system::ResMut,
};
use bevy_renet::{
    RenetServer, RenetServerPlugin, netcode::NetcodeServerPlugin, renet::DefaultChannel,
};
use deadcell_solar_conquest_shared::resources::server_configuration_factories::{
    create_server_configuration, create_server_transport_configuration,
};

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

fn send_server_message_system(mut server: ResMut<RenetServer>) {
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message".as_bytes());

    println!("I am a message being sent from the server to the client");
}
