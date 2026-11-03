use crate::systems::client::client_connection::create_client_configuration;
use bevy::app::Plugin;

pub struct ClientStartPlugin;

impl Plugin for ClientStartPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(create_client_configuration());
    }
}
