use bevy::ecs::component::Component;
use bevy_renet::renet;
use renet::ClientId;

#[derive(Debug, Component)]
pub struct Player {
    pub id: ClientId,
    pub is_on_floor: bool,
}
