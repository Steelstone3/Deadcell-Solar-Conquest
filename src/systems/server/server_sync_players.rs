use bevy::{
    ecs::system::{Query, ResMut},
    transform::components::Transform,
    utils::HashMap,
};
use bevy_renet::renet;
use renet::RenetServer;

use crate::{
    components::server::server_object::{SerializableServerObject, ServerObject},
    server::channels::GameSyncChannels,
};

pub fn server_sync(
    mut server: ResMut<RenetServer>,
    server_object_query: Query<(&Transform, &ServerObject)>,
) {
    let mut server_objects: HashMap<u32, Vec<u8>> = HashMap::new();
    for (transform, server_object) in server_object_query.iter() {
        let message =
            bincode::serialize(&SerializableServerObject::new(*transform, *server_object)).unwrap();
        server_objects.insert(server_object.id, message);
    }
    let message = bincode::serialize(&server_objects).unwrap();
    server.broadcast_message(GameSyncChannels::ServerObjects, message);
}
