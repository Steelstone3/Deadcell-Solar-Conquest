use bevy::{ecs::event::Event, transform::components::Transform};

#[derive(Event)]
pub struct SpawnPlanetEvent {
    pub sun_transform: Transform,
}
