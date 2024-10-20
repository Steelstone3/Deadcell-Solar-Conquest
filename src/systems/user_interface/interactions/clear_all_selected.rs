use bevy::{
    ecs::{event::EventWriter, system::ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    prelude::{Commands, Query},
    utils::tracing,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent, queries::selection_queries::SelectionQuery, resources::spawn_menu_selection::SpawnMenuSelection
};

pub fn clear_all_selected(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
    selection_queries: Query<SelectionQuery>,
    mut commands: Commands,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        tracing::info!("All De-Selected");

        SpawnMenuSelection::reset(&mut spawn_menu_selection);

        user_interface_event.send(UserInterfaceEvent {});

        despawn_selections(selection_queries, &mut commands);
    }
}

pub fn despawn_selections(selection_queries: Query<SelectionQuery>, commands: &mut Commands) {
    for selection_query in selection_queries.iter() {
        commands.entity(selection_query.entity).despawn();
    }
}
