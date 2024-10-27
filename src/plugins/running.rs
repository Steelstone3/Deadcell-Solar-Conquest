use crate::{
    events::event_handlers::select::handle_mouse_input,
    systems::{
        animation::animate::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        selection::{
            controllable_move_to_target::controllable_move_to_target,
            set_controllable_target::set_controllable_target,
            update_selected_sprite_location::update_selected_sprite_location,
        },
    },
};
use bevy::app::{Plugin, Update};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, camera_zoom_keyboard);
        app.add_systems(Update, camera_zoom_mouse_and_touchpad);
        app.add_systems(Update, camera_movement);
        app.add_systems(Update, camera_position_reset);
        app.add_systems(Update, handle_mouse_input);
        app.add_systems(Update, animate_sprites);
        app.add_systems(Update, set_controllable_target);
        app.add_systems(Update, controllable_move_to_target);
        app.add_systems(Update, update_selected_sprite_location);
    }
}
