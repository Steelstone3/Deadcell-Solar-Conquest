use crate::{components::space::Space, resources::player::Player};
use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec3,
};
use bevy_replicon_renet::renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    SpawnPlayer { player: Player },
    RemovePlayer { id: ClientId },
    SpawnSpace { space: Space },
    SpawnStar { entity: Entity, translation: Vec3 },
    SpawnPlanet { entity: Entity, translation: Vec3 },
    SpawnStarship { entity: Entity, translation: Vec3 },
    MoveStarship { entity: Entity, translation: Vec3 },
    DespawnStarship { entity: Entity },
    SpawnProjectile { entity: Entity, translation: Vec3 },
    DespawnProjectile { entity: Entity },
}
