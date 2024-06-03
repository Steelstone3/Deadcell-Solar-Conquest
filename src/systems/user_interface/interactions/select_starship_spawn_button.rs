use bevy::{
    color::palettes::css::{GREY, YELLOW},
    prelude::{EventWriter, Query, ResMut},
    ui::Interaction,
    utils::tracing,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectStarshipSpawnButtonQuery},
    resources::spawn_menu_selection::SpawnMenuSelection,
};

pub fn select_starship_spawn_button(
    mut select_starship_spawn_menu_button_queries: Query<
        SelectStarshipSpawnButtonQuery,
        ButtonFilters,
    >,
    mut selected_item: ResMut<SpawnMenuSelection>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_starship_spawn_menu_button_query) =
        select_starship_spawn_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_starship_spawn_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Starship Spawn Button");

            selected_item.starship_selection = select_starship_spawn_menu_button_query.button.icon;

            *select_starship_spawn_menu_button_query.border_color = YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Over Starship Spawn Button");
        }
        Interaction::None => {
            *select_starship_spawn_menu_button_query.border_color = GREY.into();
        }
    }
}
