use bevy::{
    math::{Rect, Vec2},
    prelude::{EventReader, Query},
};

use crate::{
    events::selection_events::SelectionAreaEvent, queries::user_interface_queries::SelectableQuery,
};

// TODO finish this off
pub fn select_multiple_sprites(
    mut selection_area_reader: EventReader<SelectionAreaEvent>,
    selectable_queries: Query<SelectableQuery>,
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
            // TODO select these starships
            // TODO assign the SpawnSelection resource to MultipleSelections
        }
    }
}
