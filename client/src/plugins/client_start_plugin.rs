use crate::systems::{
    camera::spawn_camera::spawn_camera,
    client::client_connection::{
        create_client_configuration, create_client_transport_configuration,
    },
};
use bevy::app::{Plugin, Startup};

pub struct ClientStartPlugin;

impl Plugin for ClientStartPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
        app.insert_resource(create_client_configuration());

        let transport = match create_client_transport_configuration() {
            Ok(transport) => transport,
            Err(_) => {
                println!("Failed to create client transport configuration");
                return;
            }
        };

        app.insert_resource(transport);
    }
}
