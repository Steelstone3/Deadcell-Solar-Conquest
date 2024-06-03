use bevy::{
    ecs::{event::EventWriter, system::ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    prelude::{Commands, Entity, Query, With},
    utils::tracing,
};

use crate::{
    components::user_interface::SelectedSprite, events::user_interface_event::UserInterfaceEvent,
    resources::spawn_menu_selection::SpawnMenuSelection,
};

pub fn deselect_all(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut selected_item: ResMut<SpawnMenuSelection>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
    selection_query: Query<Entity, With<SelectedSprite>>,
    mut commands: Commands,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        tracing::info!("All De-Selected");

        SpawnMenuSelection::reset(&mut selected_item);

        user_interface_event.send(UserInterfaceEvent {});

        for selection in selection_query.iter() {
            commands.entity(selection).despawn();
        }
    }
}
