use crate::{
    components::user_interface::{
        closest_selection::ClosestSelection, selection::SelectedSprite, tracking::Tracking,
    },
    events::{
        input_events::MouseLeftClickEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
    queries::user_interface_queries::{SelectableQuery, SelectionQuery, TypeCheckQuery},
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};
use bevy::{
    ecs::message::{MessageReader, MessageWriter},
    log::info,
    math::Vec3Swizzles,
    prelude::{Commands, In, Query, ResMut},
};

pub fn select_sprite(
    mut select_event_reader: MessageReader<MouseLeftClickEvent>,
    selectable_queries: Query<SelectableQuery>,
    mut spawn_sprite_writer: MessageWriter<SpawnSpriteEvent>,
    mut commands: Commands,
    selection_queries: Query<SelectionQuery>,
) -> Result<ClosestSelection, ()> {
    let mut closest = ClosestSelection::default();

    let Some(select) = select_event_reader.read().last() else {
        return Err(());
    };

    let cursor_position = select.cursor_world_position;

    //get list of selectables that are in range of mouse cursor
    for selectable in selectable_queries.iter() {
        let Some(size) = selectable.sprite.custom_size else {
            return Err(());
        };

        //get bounds of sprite
        let mut transform = selectable.transform.to_owned();
        let x_min = transform.translation.x - size.x / 2.0;
        let x_max = transform.translation.x + size.x / 2.0;
        let y_min = transform.translation.y - size.y / 2.0;
        let y_max = transform.translation.y + size.y / 2.0;

        //set transform to be one higher than selected so it appears above it
        transform.translation.z = 6.0;

        //if cursor is inside of the sprite bounds
        if cursor_position.x >= x_min
            && cursor_position.x <= x_max
            && cursor_position.y >= y_min
            && cursor_position.y <= y_max
        {
            //we only want to select whatever is closest to the cursor not everything underneath
            let distance = selectable
                .transform
                .translation
                .xy()
                .distance(cursor_position);
            if distance <= closest.distance || closest.distance == -1.0 {
                closest = ClosestSelection::new(
                    *selectable.transform,
                    selectable.sprite.clone(),
                    distance,
                    selectable.entity,
                );
            }
        }
    }

    //if valid selection found then spawn selection
    if closest.distance != -1.0 {
        // clear previous selected
        for selection_query in selection_queries.iter() {
            if let Some(selected_entity) = selection_query.entity {
                commands.entity(selected_entity).despawn();
            }
        }

        let selection = SelectedSprite::default();
        let selection_entity = commands
            .spawn(selection)
            .insert(Tracking {
                entity_to_follow: closest.entity,
            })
            .id();

        let Some(size) = closest.sprite.custom_size else {
            return Err(());
        };

        spawn_sprite_writer.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: selection.sprite_path.to_string(),
            size,
            transform: closest.transform,
            entity: selection_entity,
        }));
    }

    Ok(closest)
}

pub fn set_selection_type(
    In(closest_selection): In<Result<ClosestSelection, ()>>,
    type_check_query: Query<TypeCheckQuery>,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
) {
    //if closest sprite was found
    if let Ok(closest_selection) = closest_selection {
        //Detmine the type of selection for the ui
        if let Ok(selection_type) = type_check_query.get(closest_selection.entity) {
            if selection_type.starbase.is_some() {
                spawn_menu_selection.default_selection();

                spawn_menu_selection.selection = SpawnSelection::Starbase;
                info!("Starship Construction Yard Selected");
            } else if selection_type.starship.is_some() {
                spawn_menu_selection.default_selection();

                info!("Unhandled ship type selected");
                spawn_menu_selection.selection = SpawnSelection::Other;
            } else {
                spawn_menu_selection.default_selection();

                spawn_menu_selection.selection = SpawnSelection::Other;
                info!("Other Selected");
            }

            spawn_menu_selection.single_selection(closest_selection.entity);
        };
    }
}
