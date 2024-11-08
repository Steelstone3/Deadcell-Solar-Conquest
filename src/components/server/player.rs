use bevy::ecs::component::Component;
use bevy_renet::renet;
use renet::ClientId;

#[allow(dead_code)]
#[derive(Debug, Component)]
pub struct Player {
    pub id: ClientId,
    pub is_on_floor: bool,
}
