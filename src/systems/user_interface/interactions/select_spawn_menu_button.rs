use bevy::{
    color::palettes::css::{GREY, YELLOW},
    prelude::{EventWriter, Query},
    ui::Interaction,
    utils::tracing,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectSpawnMenuButtonQuery},
};

pub fn select_starship_spawn_menu_button(
    mut select_starship_spawn_menu_button_queries: Query<SelectSpawnMenuButtonQuery, ButtonFilters>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_starship_spawn_menu_button_query) =
        select_starship_spawn_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_starship_spawn_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Spawn Menu Button");

            *select_starship_spawn_menu_button_query.border_color = YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Over Spawn Menu Button");

            *select_starship_spawn_menu_button_query.border_color = YELLOW.into();
        }
        Interaction::None => {
            *select_starship_spawn_menu_button_query.border_color = GREY.into();
        }
    }
}
