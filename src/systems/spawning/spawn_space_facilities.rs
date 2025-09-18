use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::{Query, Res},
    transform::components::Transform,
};
use rand::Rng;

use crate::{
    assets::images::space_facility_type::SpaceFacilityType,
    components::{faction::space_facility::SpaceFacility, user_interface::selection::Selectable},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    queries::space_queries::StarQuery,
    resources::faction::PlayerFaction,
};

pub fn spawn_space_facilities(
    mut commands: Commands,
    star_queries: Query<StarQuery>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_faction: Res<PlayerFaction>,
) {
    for star_query in star_queries.iter() {
        let mut rng = rand::thread_rng();
        let angle = 360.0 / rng.gen_range(1.0..4.0) as f32;
        let space_station = SpaceFacility::new(
            SpaceFacilityType::SpaceStation.sprite_convert_from(player_faction.player_faction),
        );

        let x = star_query.transform.translation.x
            + star_query.star.size_component.size.x
            + space_station.size_component.size.x * 1.5;
        let y = star_query.transform.translation.y
            + star_query.star.size_component.size.x
            + space_station.size_component.size.y * 1.5;

        let transform = Transform::from_xyz(x, y, space_station.size_component.z_index)
            .with_rotation(Quat::from_rotation_z(angle.to_radians()));

        spawn_sprite_event.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
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
