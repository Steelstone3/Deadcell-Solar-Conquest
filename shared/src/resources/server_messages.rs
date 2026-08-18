use crate::resources::factions::Factions;
use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec3,
};
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    CreatePlayer { id: ClientId, faction: Factions },
    RemovePlayer { id: ClientId },
    SpawnSpace { entity: Entity, translation: Vec3 },
    SpawnStar { entity: Entity, translation: Vec3 },
    SpawnPlanet { entity: Entity, translation: Vec3 },
    SpawnStarship { entity: Entity, translation: Vec3 },
    DespawnStarship { entity: Entity },
    SpawnProjectile { entity: Entity, translation: Vec3 },
    DespawnProjectile { entity: Entity },
}
