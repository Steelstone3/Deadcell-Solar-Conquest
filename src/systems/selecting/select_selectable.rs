use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query},
    },
    sprite::Sprite,
    transform::components::Transform,
};
use rand::random;

use crate::{
    components::{selectable::Selectable, selection::Selection},
    events::{mouse_click_event::MouseClickEvent, spawn_sprite_event::SpawnSpriteEvent},
};

pub fn select_selectable(
    mut select_event_reader: EventReader<MouseClickEvent>,
    selectable_query: Query<(&Transform, &Selectable, &Sprite)>,
    mut spawn_sprite_writer: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
    selection_query: Query<Entity, With<Selection>>,
) {
    let Some(event) = select_event_reader.read().last() else {
        return;
    };

    for space_station in selectable_query.iter() {
        let cursor_position = event.cursor_world_position;
        let Some(size) = space_station.2.custom_size else {
            return;
        };

        let mut transform = space_station.0.to_owned();
        let x_min = transform.translation.x - size.x / 2.0;
        let x_max = transform.translation.x + size.x / 2.0;
        let y_min = transform.translation.y - size.y / 2.0;
        let y_max = transform.translation.y + size.y / 2.0;

        transform.translation.z += 1.0;

        if cursor_position.x >= x_min
            && cursor_position.x <= x_max
            && cursor_position.y >= y_min
            && cursor_position.y <= y_max
        {
            let selection = Selection::new(random());
            let selection_entity = commands.spawn(selection).id();

            spawn_sprite_writer.send(SpawnSpriteEvent {
                sprite_path: selection.sprite_path.to_string(),
                size,
                transform,
                entity: selection_entity,
            });

            for selection in selection_query.iter() {
                commands.entity(selection).despawn();
            }
        } else {
            for selection in selection_query.iter() {
                commands.entity(selection).despawn();
            }
        }
    }
}
