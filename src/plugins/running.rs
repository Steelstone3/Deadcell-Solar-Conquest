use crate::{
    events::event_handlers::handle_mouse_input::{
        handle_left_click, handle_left_click_with_modifier, handle_right_click,
    },
    systems::{
        animation::animate::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        selection::movement::{
            move_to_point::move_to_point,
            set_destination::{set_multiple_destination, set_single_destination},
            update_selected_sprite_destination::update_selected_sprite_destination,
        },
        server::{
            server_sync_players::server_sync, server_update::server_update,
            server_update_positions::server_update_positions,
        },
    },
};
use bevy::{
    app::{Plugin, Update},
    prelude::{resource_exists, IntoSystemConfigs},
};
use bevy_renet::renet::RenetServer;

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, camera_zoom_keyboard);
        app.add_systems(Update, camera_zoom_mouse_and_touchpad);
        app.add_systems(Update, camera_movement);
        app.add_systems(Update, camera_position_reset);
        app.add_systems(Update, handle_left_click);
        app.add_systems(Update, handle_left_click_with_modifier);
        app.add_systems(Update, handle_right_click);
        app.add_systems(Update, animate_sprites);
        app.add_systems(Update, set_single_destination);
        app.add_systems(Update, set_multiple_destination);
        app.add_systems(Update, move_to_point);
        app.add_systems(Update, update_selected_sprite_destination);
        app.add_systems(
            Update,
            (server_update, server_sync, server_update_positions)
                .run_if(resource_exists::<RenetServer>),
        );
    }
}
