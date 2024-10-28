use bevy::{
    math::{Rect, Vec2},
    prelude::{Commands, EventReader, EventWriter, Query, ResMut},
};

use crate::{
    components::user_interface::{selection::SelectedSprite, tracking::Tracking},
    events::{
        selection_events::SelectionAreaEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
    queries::user_interface_queries::{SelectableQuery, SelectionQuery, TypeCheckQuery},
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

// TODO finish this off
pub fn select_multiple_sprites(
    mut commands: Commands,
    mut selection_area_reader: EventReader<SelectionAreaEvent>,
    selection_queries: Query<SelectionQuery>,
    selectable_queries: Query<SelectableQuery>,
    type_check_query: Query<TypeCheckQuery>,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    mut spawn_sprite_writer: EventWriter<SpawnSpriteEvent>,
) {
    let Some(selection_area) = selection_area_reader.read().last() else {
        return;
    };

    // clear previous selected
    for selection_query in selection_queries.iter() {
        if let Some(selected_entity) = selection_query.entity {
            commands.entity(selected_entity).despawn();
        }
    }
    spawn_menu_selection.reset_all();

    for selectable in selectable_queries.iter() {
        // create an area out of the two cursor points
        let area = Rect::new(
            selection_area.selection_area.start.x,
            selection_area.selection_area.start.y,
            selection_area.selection_area.end.x,
            selection_area.selection_area.end.y,
        );

        let selectable_location = Vec2::new(
            selectable.transform.translation.x,
            selectable.transform.translation.y,
        );

        // check if a selectable entity exists within the area
        if area.contains(selectable_location) {
            if let Ok(selection_type) = type_check_query.get(selectable.entity) {
                // select the entity if it is a type of starship
                if selection_type.starship.is_some() {
                    spawn_menu_selection.default_selection();
                    spawn_menu_selection.selection = SpawnSelection::MultipleSelections;

                    // create a selection indicator entity
                    let selection = SelectedSprite::default();
                    let selection_entity = commands
                        .spawn(selection)
                        .insert(Tracking {
                            entity_to_follow: selectable.entity,
                        })
                        .id();

                    let Some(size) = selectable.sprite.custom_size else {
                        return;
                    };

                    // spawn the selection indicator entity
                    spawn_sprite_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
                        sprite_path: selection.sprite_path.to_string(),
                        size,
                        transform: *selectable.transform,
                        entity: selection_entity,
                    }));

                    // selected entities
                    spawn_menu_selection.multi_selection(selectable.entity);
                }
            }
        }
    }
}
