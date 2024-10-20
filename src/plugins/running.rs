use crate::{
    events::event_handlers::select::handle_mouse_input,
    systems::{
        animation::animate::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        controller::set_controllable_target::set_controllable_target,
        movement::{
            controllable_move_to_target::controllable_move_to_target,
            update_selected_sprite_location::update_selected_sprite_location,
        },
        user_interface::sprite_selection::{set_selection_type, sprite_selection},
    },
};
use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystem,
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                camera_zoom_keyboard,
                camera_zoom_mouse_and_touchpad,
                camera_movement,
                camera_position_reset,
                handle_mouse_input,
                sprite_selection.pipe(set_selection_type),
                animate_sprites,
                set_controllable_target,
                controllable_move_to_target,
                update_selected_sprite_location,
            ),
        );
    }
}
