use crate::{
    queries::user_interface_queries::SelectionQuery,
    resources::spawn_menu_selection::SpawnMenuSelection,
};
use bevy::{
    ecs::system::ResMut,
    input::{ButtonInput, keyboard::KeyCode},
    log::tracing,
    prelude::{Commands, Query},
};

pub fn clear_all_selected(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    selection_queries: Query<SelectionQuery>,
    mut commands: Commands,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        tracing::info!("All De-Selected");

        spawn_menu_selection.reset_all();

        for selection_query in selection_queries.iter() {
            if let Some(selected_entity) = selection_query.entity {
                commands.entity(selected_entity).despawn();
            }
        }
    }
}
