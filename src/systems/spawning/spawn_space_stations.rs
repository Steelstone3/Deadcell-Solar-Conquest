use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::{Query, Res},
    transform::components::Transform,
};
use rand::Rng;

use crate::{
    assets::images::space_facility_type::SpaceFacilityType,
    components::{space_facility::SpaceFacility, user_interface::Selectable},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    queries::space_queries::SunQuery,
    resources::faction::PlayerFaction,
};

pub fn spawn_space_stations(
    mut commands: Commands,
    sun_queries: Query<SunQuery>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_faction: Res<PlayerFaction>,
) {
    for sun_query in sun_queries.iter() {
        let mut rng = rand::thread_rng();
        let angle = 360.0 / rng.gen_range(1.0..4.0) as f32;
        let space_station = SpaceFacility::new(
            SpaceFacilityType::SpaceStation.sprite_convert_from(player_faction.player_faction),
        );

        let x = sun_query.transform.translation.x
            + sun_query.sun.size_component.size.x
            + space_station.size_component.size.x * 1.5;
        let y = sun_query.transform.translation.y
            + sun_query.sun.size_component.size.x
            + space_station.size_component.size.y * 1.5;

        let transform = Transform::from_xyz(x, y, space_station.size_component.z_index)
            .with_rotation(Quat::from_rotation_z(angle.to_radians()));

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
}
