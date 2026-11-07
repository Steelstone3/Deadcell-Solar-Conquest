use crate::systems::client::client_connection::{
    create_client_configuration, create_client_transport_configuration,
};
use bevy::app::Plugin;

pub struct ClientStartPlugin;

impl Plugin for ClientStartPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(create_client_configuration());

        // let transport = match create_client_transport_configuration("127.0.0.1:5000") {
        //     Ok(transport) => transport,
        //     Err(_) => return,
        // };

        // app.insert_resource(transport);
    }
}
