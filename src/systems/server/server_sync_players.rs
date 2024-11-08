use bevy::{
    ecs::system::{Query, ResMut},
    transform::components::Transform,
};
use bevy_renet::renet;
use renet::RenetServer;

pub fn server_sync(_server: ResMut<RenetServer>, _query: Query<&Transform>) {}
