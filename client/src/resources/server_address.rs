use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct ServerAddress {
    pub server_address: String,
}

impl Default for ServerAddress {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1:5000".to_string(),
        }
    }
}