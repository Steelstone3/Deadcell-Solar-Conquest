use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Entity, EventReader, EventWriter, Query, Transform},
};

use crate::{
    components::user_interface::multiple_selection_box::MultipleSelectionBox,
    events::{
        input_events::MouseLeftClickModifierEvent,
        selection_events::SelectionAreaEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
};

// TODO create queries
pub fn draw_multiple_selection_box(
    mut commands: Commands,
    mut select_event_reader: EventReader<MouseLeftClickModifierEvent>,
    mut selection_query: Query<(Entity, &mut MultipleSelectionBox)>,
    mut spawn_sprite_writer: EventWriter<SpawnSpriteEvent>,
    mut selection_area_writer: EventWriter<SelectionAreaEvent>,
) {
    let Some(select) = select_event_reader.read().last() else {
        return;
    };

    let cursor_position = select.cursor_world_position;

    match !selection_query.is_empty() {
        true => match !select.just_released {
            true => {
                // Resize Selection Box
                let selection_box = selection_query.single_mut();
                let mut multiple_selection_box = selection_box.1;
                multiple_selection_box.selection_area.end = cursor_position;

                let midpoint = multiple_selection_box
                    .selection_area
                    .start
                    .lerp(multiple_selection_box.selection_area.end, 0.5)
                    .extend(50.0);
                let size_x = multiple_selection_box.selection_area.end.x
                    - multiple_selection_box.selection_area.start.x;
                let size_y = multiple_selection_box.selection_area.end.y
                    - multiple_selection_box.selection_area.start.y;

                commands
                    .entity(selection_box.0)
                    .insert(*multiple_selection_box)
                    .insert(
                        Transform::from_translation(midpoint)
                            .with_scale(Vec3::new(size_x, size_y, 6.0)),
                    );
            }
            false => {
                let selection_box = selection_query.single_mut();

                selection_area_writer.send(SelectionAreaEvent {
                    selection_area: selection_box.1.selection_area,
                });

                commands.entity(selection_box.0).despawn();
            }
        },
        false => {
            // Create Selection Box
            let multiple_selection_box = MultipleSelectionBox::new(cursor_position);

            let midpoint = (multiple_selection_box.selection_area.start
                + multiple_selection_box.selection_area.end)
                .extend(10.0)
                / 2.0;

            let entity = commands.spawn(multiple_selection_box).id();

            spawn_sprite_writer.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
                sprite_path: multiple_selection_box.sprite_path.to_string(),
                size: Vec2 { x: 1.0, y: 1.0 },
                transform: Transform::from_translation(midpoint),
                entity,
            }));
        }
    }
}
