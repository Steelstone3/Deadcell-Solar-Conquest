use bevy::prelude::EventReader;

use crate::events::selection_events::SelectionAreaEvent;

// TODO finish this off
pub fn select_multiple_sprites(mut selection_area_reader: EventReader<SelectionAreaEvent>) {
    let Some(_selection_area) = selection_area_reader.read().last() else {
        return;
    };

    // TODO at this point the selection box has despawned (approximately) with the area the box sent in this event
    // TODO use this area to determine which starships exist within it (inclusive for those on the border)
    // TODO select these starships
    // TODO assign the SpawnSelection resource to MultipleSelections
}
