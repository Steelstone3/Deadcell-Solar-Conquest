use crate::systems::server::server_connection::{
    create_server_configuration, create_server_transport_configuration,
};
use bevy::app::{App, Plugin};

pub struct ServerStartPlugin;

impl Plugin for ServerStartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_server_configuration());

        let transport = match create_server_transport_configuration() {
            Ok(transport) => transport,
            Err(_) => return,
        };

        app.insert_resource(transport);
    }
}
