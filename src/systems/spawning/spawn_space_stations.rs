use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::Res,
    transform::components::Transform,
};
use rand::Rng;

use crate::{
    components::{space_station::SpaceStation, user_interface::Selectable},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    resources::{
        constants::SPACE_STATION_DISTANCE_FROM_CENTRE,
        faction::{PlayerFaction, StarStationType},
    },
};

pub fn spawn_space_stations(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_faction: Res<PlayerFaction>,
) {
    let mut rng = rand::thread_rng();
    let angle = 360.0 / rng.gen_range(1.0..4.0) as f32;
    let space_station = SpaceStation::new(
        StarStationType::SpaceStation.sprite_convert_from(player_faction.player_faction),
    );
    let mut transform = Transform::from_xyz(0.0, 0.0, space_station.size_component.z_index)
        .with_rotation(Quat::from_rotation_z(angle.to_radians()));

    transform.translation += transform.up() * SPACE_STATION_DISTANCE_FROM_CENTRE;

    spawn_sprite_event.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
        sprite_path: space_station.sprite_path.to_string(),
        size: space_station.size_component.size,
        transform,
        entity: commands
            .spawn(space_station)
            .insert(Selectable)
            .insert(transform)
            .id(),
    }));
}
