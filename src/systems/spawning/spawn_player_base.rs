use bevy::{
    audio::CpalSample,
    ecs::{
        event::EventWriter,
        system::{Commands, Res},
    },
    math::Quat,
    transform::components::Transform,
};
use rand::random;

use crate::{
    components::{selectable::Selectable, space_station::SpaceStation},
    events::{spawn_planet_event::SpawnPlanetEvent, spawn_sprite_event::SpawnSpriteEvent},
    resources::{constants::SPACE_STATION_DISTANCE_FROM_CENTRE, game_settings::GameSettings},
};

pub fn spawn_space_stations(
    game_settings: Res<GameSettings>,
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    mut spawn_planet_event: EventWriter<SpawnPlanetEvent>,
) {
    let angle = 360.0 / game_settings.number_of_players.to_float_sample();

    for player in 0..game_settings.number_of_players {
        let space_station = SpaceStation::new(random());
        let current_angle = angle * player.to_float_sample();
        let mut transform = Transform::from_xyz(0.0, 0.0, space_station.z_index)
            .with_rotation(Quat::from_rotation_z(current_angle.to_radians()));

        transform.translation += transform.up() * SPACE_STATION_DISTANCE_FROM_CENTRE;

        spawn_sprite_event.send(SpawnSpriteEvent {
            sprite_path: space_station.sprite_path.to_string(),
            size: space_station.size,
            transform,
            entity: commands.spawn(space_station).insert(Selectable).id(),
        });

        spawn_planet_event.send(SpawnPlanetEvent {
            space_station_transform: transform,
        });
    }
}
