use bevy::{ecs::system::Resource, input::keyboard::KeyCode};

// TODO Some of the variables aren't used
#[derive(Resource)]
pub struct KeyBindings {
    pub camera_slow: Vec<KeyCode>,
    pub camera_fast: Vec<KeyCode>,
    pub camera_up: Vec<KeyCode>,
    pub camera_down: Vec<KeyCode>,
    pub camera_left: Vec<KeyCode>,
    pub camera_right: Vec<KeyCode>,
    #[allow(dead_code)]
    pub camera_reset_position: Vec<KeyCode>,
    #[allow(dead_code)]
    pub camera_reset_zoom: Vec<KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            camera_slow: [KeyCode::ControlLeft, KeyCode::ControlRight].to_vec(),
            camera_fast: [KeyCode::ShiftLeft, KeyCode::ShiftRight].to_vec(),
            camera_up: [KeyCode::KeyW, KeyCode::ArrowUp].to_vec(),
            camera_down: [KeyCode::KeyS, KeyCode::ArrowDown].to_vec(),
            camera_left: [KeyCode::KeyA, KeyCode::ArrowLeft].to_vec(),
            camera_right: [KeyCode::KeyD, KeyCode::ArrowRight].to_vec(),
            camera_reset_position: [KeyCode::Home].to_vec(),
            camera_reset_zoom: [KeyCode::KeyR].to_vec(),
        }
    }
}
