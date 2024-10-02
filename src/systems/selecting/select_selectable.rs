use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query},
    },
    math::Vec3Swizzles,
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

    let cursor_position = event.cursor_world_position;
    let mut selectables: Vec<(&Transform, &Selectable, &Sprite)> = Vec::new();

    //get list of selectables that are in range of mouse cursor
    for selectable in selectable_query.iter() {
        let Some(size) = selectable.2.custom_size else {
            return;
        };

        let mut transform = selectable.0.to_owned();
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
            selectables.push(selectable);
        }
    }

    let mut closest = (
        &Transform::from_xyz(0.0, 0.0, 0.0),
        &Sprite::default(),
        -1.0,
    );

    //get the closest selectable
    for selectable in selectables {
        let distance = selectable.0.translation.xy().distance(cursor_position);
        if distance <= closest.2 || closest.2 == -1.0 {
            closest = (selectable.0, selectable.2, distance);
        }
    }

    //Clear selection before makeing new selection
    for selection in selection_query.iter() {
        commands.entity(selection).despawn();
    }

    //if valid selection found then spawn selection
    if closest.2 != -1.0 {
        let selection = Selection::new(random());
        let selection_entity = commands.spawn(selection).id();

        let Some(size) = closest.1.custom_size else {
            return;
        };

        spawn_sprite_writer.send(SpawnSpriteEvent {
            sprite_path: selection.sprite_path.to_string(),
            size,
            transform: *closest.0,
            entity: selection_entity,
        });
    }
}
