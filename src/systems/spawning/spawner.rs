use bevy::{
    ecs::{event::EventWriter, system::Commands},
    prelude::{EventReader, Res},
    transform::components::Transform,
    utils::tracing,
};

use crate::{
    assets::{
        images::faction_starship_sprite::starship_sprite::StarshipSprite,
        user_interface::icons::{
            space_facility_icons::SpaceFacilityIcon, starship_icons::StarshipIcon,
        },
    },
    components::{
        controllable::Movement,
        space_facility::SpaceFacility,
        starship::{ShipSpeed, Starship},
        user_interface::Selectable,
    },
    events::{
        mouse_right_click_event::MouseRightClickEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

pub fn spawner(
    mut commands: Commands,
    spawn_menu_selection: Res<SpawnMenuSelection>,
    mut right_mouse_events: EventReader<MouseRightClickEvent>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
) {
    right_mouse_events.read().for_each(|event| {
        // let mut transform = Transform::default();
        let mut transform = Transform {
            translation: event.cursor_world_position.extend(5.0),
            ..Default::default()
        };

        match spawn_menu_selection.selection {
            SpawnSelection::None => {}
            SpawnSelection::Other => {}
            SpawnSelection::MultiSelection => {}
            SpawnSelection::StarshipConstructionYard => {
                spawn_starship(
                    &mut transform,
                    &spawn_menu_selection,
                    &mut spawn_sprite_event,
                    &mut commands,
                );
            }
            SpawnSelection::SupportShip => {
                spawn_space_facility(
                    &mut transform,
                    &spawn_menu_selection,
                    &mut spawn_sprite_event,
                    &mut commands,
                );
            }
            SpawnSelection::Starbase => {
                spawn_starship(
                    &mut transform,
                    &spawn_menu_selection,
                    &mut spawn_sprite_event,
                    &mut commands,
                );
            }
        }
    });
}

fn spawn_space_facility(
    transform: &mut Transform,
    selected_item: &Res<'_, SpawnMenuSelection>,
    spawn_sprite_event: &mut EventWriter<'_, SpawnSpriteEvent>,
    commands: &mut Commands<'_, '_>,
) {
    tracing::info!("space facility at {:?}", transform.translation);

    if selected_item.space_facility_selection != SpaceFacilityIcon::None {
        let space_facility = SpaceFacility::new_from_icon(selected_item.space_facility_selection);
        transform.translation.z = space_facility.size_component.z_index;

        spawn_sprite_event.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: space_facility.sprite_path.to_string(),
            size: space_facility.size_component.size,
            transform: *transform,
            entity: commands.spawn(space_facility).insert(Selectable).id(),
        }));
    }
}

fn spawn_starship(
    transform: &mut Transform,
    selected_item: &Res<'_, SpawnMenuSelection>,
    spawn_sprite_event: &mut EventWriter<'_, SpawnSpriteEvent>,
    commands: &mut Commands<'_, '_>,
) {
    tracing::info!("starship at {:?}", transform.translation);

    if selected_item.starship_selection != StarshipIcon::None {
        let starship = Starship::new_from_icon(selected_item.starship_selection);

        let ship_speed = ShipSpeed::new_from_ship_type(StarshipSprite::starship_type_convert_from(
            starship.starship_sprite_bundle.starship_sprite,
        ));

        transform.translation.z = starship.size_component.z_index;

        spawn_sprite_event.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starship.starship_sprite_bundle.starship_sprite.to_string(),
            size: starship.size_component.size,
            transform: *transform,
            entity: commands
                .spawn(starship)
                .insert(Selectable)
                .insert(Movement {
                    target_location: transform.translation,
                    maximum_speed: ship_speed.speed,
                    current_speed: 0.0,
                })
                .id(),
        }));
    }
}
