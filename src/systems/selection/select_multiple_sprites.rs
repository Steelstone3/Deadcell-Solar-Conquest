use bevy::{
    math::{Rect, Vec2},
    prelude::{Commands, EventReader, EventWriter, Query, ResMut},
};

use crate::{
    components::user_interface::{selection::SelectedSprite, tracking::Tracking}, events::{selection_events::SelectionAreaEvent, spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent}}, queries::user_interface_queries::{SelectableQuery, TypeCheckQuery}, resources::spawn_menu_selection::SpawnMenuSelection, systems::user_interface::interactions::spawn_selection::SpawnSelection
};

// TODO finish this off
pub fn select_multiple_sprites(
    mut commands: Commands,
    mut selection_area_reader: EventReader<SelectionAreaEvent>,
    selectable_queries: Query<SelectableQuery>,
    type_check_query: Query<TypeCheckQuery>,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    mut spawn_sprite_writer: EventWriter<SpawnSpriteEvent>,
) {
    let Some(selection_area) = selection_area_reader.read().last() else {
        return;
    };

    for selectable in selectable_queries.iter() {
        // let Some(size) = selectable.sprite.custom_size else {
        //     return;
        // };

        // TODO at this point the selection box has despawned (approximately) with the area the box sent in this event

        // TODO use this area to determine which starships exist within it (inclusive for those on the border)

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

        if area.contains(selectable_location) {
            // TODO For starships only
            SpawnMenuSelection::default_selection(&mut spawn_menu_selection);
            spawn_menu_selection.selection = SpawnSelection::MultipleSelections;


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

            spawn_sprite_writer.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
                sprite_path: selection.sprite_path.to_string(),
                size,
                transform: *selectable.transform,
                entity: selection_entity,
            }));
        
            // TODO select these starships
            // TODO assign the SpawnSelection resource to MultipleSelections
        }
    }
}
