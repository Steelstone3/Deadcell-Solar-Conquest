use bevy::{ecs::component::Component, prelude::Transform};
use serde::{Deserialize, Serialize};

//Carnt sync entity directly so use dummy id object instead
//For use with objects that need to be synced between server and client every frame
#[derive(Debug, Serialize, Deserialize, Component, Clone, Copy, PartialEq, Eq)]
pub struct ServerObject {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SerializableServerObject {
    pub server_object: ServerObject,
    pub transform: Transform,
}

impl SerializableServerObject {
    pub fn new(transform: Transform, server_object: ServerObject) -> Self {
        Self {
            transform,
            server_object,
        }
    }
}
