use bevy::{
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SpawnSubMenuQuery,
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

pub fn despawn_sub_menus(
    selected_item: Res<SpawnMenuSelection>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SpawnSubMenuQuery>,
    mut commands: Commands,
) {
    for _ in user_interface_events.read() {
        if selected_item.selection == SpawnSelection::None {
            // Remove UI
            if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
                commands.entity(sub_menu_query.entity).despawn_recursive();
            }
        }
    }
}
