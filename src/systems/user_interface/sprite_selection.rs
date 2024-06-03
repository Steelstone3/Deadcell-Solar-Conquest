use crate::{
    assets::images::faction_starship_sprite::starship_sprite::StarshipSprite,
    components::{
        space_facility::SpaceFacility,
        space_station::SpaceStation,
        starship::Starship,
        tracking::Tracking,
        user_interface::{Selectable, SelectedSprite},
    },
    events::{
        mouse_click_event::MouseClickEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
    resources::{faction::StarshipType, spawn_menu_selection::SpawnMenuSelection},
};
use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query},
    },
    log::info,
    math::Vec3Swizzles,
    prelude::ResMut,
    sprite::Sprite,
    transform::components::Transform,
};
use rand::random;

use super::interactions::spawn_selection::SpawnSelection;

pub fn sprite_selection(
    mut select_event_reader: EventReader<MouseClickEvent>,
    selectable_query: Query<(&Transform, &Sprite, Entity), With<Selectable>>,
    mut spawn_sprite_writer: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
    selection_queries: Query<Entity, With<SelectedSprite>>,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    type_check_query: Query<(
        Option<&SpaceStation>,
        Option<&SpaceFacility>,
        Option<&Starship>,
    )>,
) {
    let Some(event) = select_event_reader.read().last() else {
        return;
    };

    let cursor_position = event.cursor_world_position;

    let mut closest = (
        &Transform::from_xyz(0.0, 0.0, 0.0),
        &Sprite::default(),
        -1.0,
        Entity::PLACEHOLDER,
    );

    //get list of selectables that are in range of mouse cursor
    for selectable in selectable_query.iter() {
        let Some(size) = selectable.1.custom_size else {
            return;
        };

        //get bounds of sprite
        let mut transform = selectable.0.to_owned();
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
            //we only want to select whatevers closest to the cursor not everything undeneath
            let distance = selectable.0.translation.xy().distance(cursor_position);
            if distance <= closest.2 || closest.2 == -1.0 {
                closest = (selectable.0, selectable.1, distance, selectable.2);
            }
        }
    }

    //if valid selection found then spawn selection
    if closest.2 != -1.0 {
        //Clear selection before makeing new selection
        for selection in selection_queries.iter() {
            commands.entity(selection).despawn();
        }

        let selection = SelectedSprite::new(random());
        let selection_entity = commands
            .spawn(selection)
            .insert(Tracking {
                entity_to_follow: closest.3,
            })
            .id();

        let Some(size) = closest.1.custom_size else {
            return;
        };

        spawn_sprite_writer.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: selection.sprite_path.to_string(),
            size,
            transform: *closest.0,
            entity: selection_entity,
        }));

        //Detmine the type of selection for the ui
        if let Ok(selection_type) = type_check_query.get(closest.3) {
            if let Some(_space_station) = selection_type.0 {
                spawn_menu_selection.selection = SpawnSelection::Starbase;
                info!("Starbase Selected");
            } else if let Some(_space_facility) = selection_type.1 {
                spawn_menu_selection.selection = SpawnSelection::StarshipConstructionYard;
                info!("Starship Construction Yard Selected");
            } else if let Some(spaceship) = selection_type.2 {
                let spaceship_type = StarshipSprite::starship_type_convert_from(
                    spaceship.starship_sprite_bundle.starship_sprite,
                );
                if spaceship_type == StarshipType::SupportShip {
                    spawn_menu_selection.selection = SpawnSelection::SupportShip;
                    info!("Support Ship Selected");
                } else {
                    info!("Unhandled ship type {spaceship_type} selected");
                    spawn_menu_selection.selection = SpawnSelection::Other;
                }
            } else {
                spawn_menu_selection.selection = SpawnSelection::Other;
                info!("Other Selected");
            }

            spawn_menu_selection.selected_entity = closest.3;
        };
    }
}
