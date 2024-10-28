use crate::{
    components::user_interface::multiple_selection_box::MultipleSelectionBox,
    events::{
        input_events::MouseLeftClickModifierEvent,
        selection_events::SelectionAreaEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
    queries::selection_queries::SelectionBoxQuery,
};
use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, EventReader, EventWriter, Query, Transform},
};

// TODO create queries
pub fn draw_multiple_selection_box(
    mut commands: Commands,
    mut select_event_reader: EventReader<MouseLeftClickModifierEvent>,
    mut selection_query: Query<SelectionBoxQuery>,
    mut spawn_sprite_writer: EventWriter<SpawnSpriteEvent>,
    mut selection_area_writer: EventWriter<SelectionAreaEvent>,
) {
    let Some(select) = select_event_reader.read().last() else {
        return;
    };

    let entity = commands.spawn(()).id();
    let cursor_position = select.cursor_world_position;

    match !selection_query.is_empty() {
        true => match !select.just_released {
            true => {
                // Resize Selection Box
                let selection_box = selection_query.single_mut();

                match selection_box {
                    Ok(mut selection_box) => {
                        selection_box.multiple_selection_box.selection_area.end = cursor_position;

                        let midpoint = selection_box
                            .multiple_selection_box
                            .selection_area
                            .start
                            .lerp(selection_box.multiple_selection_box.selection_area.end, 0.5)
                            .extend(50.0);
                        let size_x = selection_box.multiple_selection_box.selection_area.end.x
                            - selection_box.multiple_selection_box.selection_area.start.x;
                        let size_y = selection_box.multiple_selection_box.selection_area.end.y
                            - selection_box.multiple_selection_box.selection_area.start.y;

                        commands
                            .entity(entity)
                            .insert(*selection_box.multiple_selection_box)
                            .insert(
                                Transform::from_translation(midpoint)
                                    .with_scale(Vec3::new(size_x, size_y, 6.0)),
                            );
                    }
                    Err(_) => {}
                }
            }
            false => {
                let selection_box = selection_query.single_mut();

                match selection_box {
                    Ok(selection_box) => {
                        selection_area_writer.write(SelectionAreaEvent {
                            selection_area: selection_box.multiple_selection_box.selection_area,
                        });

                        commands.entity(entity).despawn();
                    }
                    Err(_) => {}
                }
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

            spawn_sprite_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
                sprite_path: multiple_selection_box.sprite_path.to_string(),
                size: Vec2 { x: 1.0, y: 1.0 },
                transform: Transform::from_translation(midpoint),
                entity,
            }));
        }
    }
}
