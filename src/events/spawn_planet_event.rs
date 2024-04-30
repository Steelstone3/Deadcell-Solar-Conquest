use bevy::{ecs::event::Event, transform::components::Transform};

#[derive(Event)]
pub struct SpawnPlanetEvent {
    pub space_station_transform: Transform,
}
