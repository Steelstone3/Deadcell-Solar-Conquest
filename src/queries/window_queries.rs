use bevy::{ecs::query::QueryData, window::Window};

#[derive(QueryData)]
pub struct WindowQuery {
    pub window: &'static Window,
}
