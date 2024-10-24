use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::Res,
    transform::components::Transform,
};
use rand::Rng;

use crate::{
    assets::images::space_facility_type::SpaceFacilityType,
    components::{space_facility::SpaceFacility, user_interface::Selectable},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    resources::{constants::SPACE_TILE_SIZE, faction::PlayerFaction},
};

// TODO This will be based on planet/ sun location later
const DISTANCE_FROM_CENTRE: f32 = SPACE_TILE_SIZE * 5.0;

pub fn spawn_space_stations(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_faction: Res<PlayerFaction>,
) {
    let mut rng = rand::thread_rng();
    let angle = 360.0 / rng.gen_range(1.0..4.0) as f32;
    let space_station = SpaceFacility::new(
        SpaceFacilityType::SpaceStation.sprite_convert_from(player_faction.player_faction),
    );
    let mut transform = Transform::from_xyz(0.0, 0.0, space_station.size_component.z_index)
        .with_rotation(Quat::from_rotation_z(angle.to_radians()));

    transform.translation += transform.up() * DISTANCE_FROM_CENTRE;

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
