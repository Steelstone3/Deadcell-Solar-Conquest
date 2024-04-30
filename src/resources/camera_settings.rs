use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct CameraSettings {
    pub zoom_speed: f32,
    pub minimum_zoom: f32,
    pub maximum_zoom: f32,
    pub current_zoom: f32,
    pub zoom_in: f32,
    pub zoom_out: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            zoom_speed: 1.5,
            minimum_zoom: 0.5,
            maximum_zoom: 5.0,
            current_zoom: 3.0,
            zoom_in: 1.1,
            zoom_out: 0.9,
        }
    }
}
