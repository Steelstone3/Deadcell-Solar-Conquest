use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SelectionQuery,
    resources::spawn_menu_selection::SpawnMenuSelection,
};
use bevy::{
    ecs::{event::EventWriter, system::ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    prelude::{Commands, Query},
    utils::tracing,
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

        SpawnMenuSelection::reset_all(&mut spawn_menu_selection);

        user_interface_event.send(UserInterfaceEvent {});

        for selection_query in selection_queries.iter() {
            if let Some(selected_entity) = selection_query.entity {
                commands.entity(selected_entity).despawn();
            }
        }
    }
}
