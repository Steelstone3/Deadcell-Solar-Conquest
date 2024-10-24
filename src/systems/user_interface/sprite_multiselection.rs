use crate::{
    assets::user_interface::multiselect::MultiSelectSprite,
    components::multiselect::MultiSelect,
    events::{
        mouse_click_event::MouseClickEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
};
use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Entity, EventReader, EventWriter, Query, Transform},
};

pub fn sprite_multiselection(
    mut select_event_reader: EventReader<MouseClickEvent>,
    mut commands: Commands,
    mut selection_query: Query<(Entity, &mut MultiSelect)>,
    mut spawn_sprite_writer: EventWriter<SpawnSpriteEvent>,
) {
    let Some(event) = select_event_reader.read().last() else {
        return;
    };

    //if ctrl isnt pressed then let single select handle
    if !event.ctrl_modifier {
        if !selection_query.is_empty() {
            let selection_box = selection_query.single_mut();
            commands.entity(selection_box.0).despawn();
        }
        return;
    }

    let cursor_position = event.cursor_world_position;
    let selection_sprite = MultiSelectSprite::MultiSelectNeutral;

    if !selection_query.is_empty() {
        let selection_box = selection_query.single_mut();
        if !event.just_released {
            let mut multiselect = selection_box.1;
            multiselect.end = cursor_position;

            let midpoint = multiselect.start.lerp(multiselect.end, 0.5).extend(50.0);
            let size_x = multiselect.end.x - multiselect.start.x;
            let size_y = multiselect.end.y - multiselect.start.y;

            commands
                .entity(selection_box.0)
                .insert(*multiselect)
                .insert(
                    Transform::from_translation(midpoint)
                        .with_scale(Vec3::new(size_x, size_y, 1.0)),
                );
        }
        //remove multiselect once mouse is released
        //TODO Matthew pipe to next system to actually select ships in selection
        else {
            commands.entity(selection_box.0).despawn();
        }
    } else {
        let multiselect = MultiSelect {
            start: cursor_position,
            end: cursor_position,
        };
        let midpoint = (multiselect.start + multiselect.end).extend(10.0) / 2.0;

        let entity = commands.spawn(multiselect).id();

        spawn_sprite_writer.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: selection_sprite.to_string(),
            size: Vec2 { x: 1.0, y: 1.0 },
            transform: Transform::from_translation(midpoint),
            entity,
        }));
    }
}
