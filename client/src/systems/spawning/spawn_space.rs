use bevy::ecs::{message::MessageWriter, system::{Commands, Query}};

use crate::{events::spawn_sprite_event::SpawnSpriteEvent, systems::queries::space_queries::SpaceTransformQuery};

pub fn spawn_space(mut command: Commands, mut spawn_sprite_event: MessageWriter<SpawnSpriteEvent>,   space_transform_queries: Query<SpaceTransformQuery>,) {
    for space_transform_query in space_transform_queries {
        println!("I, as space, exist")
    }

}
