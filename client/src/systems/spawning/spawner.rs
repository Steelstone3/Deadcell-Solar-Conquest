use crate::{
    assets::images::starship_sprite::{StarshipSprite, StarshipType},
    components::{
        faction::starship::{Starship, StarshipSpeed},
        server::server_object::ServerObject,
        user_interface::{controllable::Movement, selection::Selectable},
    },
    events::{
        input_events::MouseRightClickEvent,
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    },
    resources::{faction::PlayerFaction, spawn_menu_selection::SpawnMenuSelection},
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};
use bevy::{
    ecs::{message::{MessageReader, MessageWriter}, system::Commands}, log::tracing, prelude::Res, transform::components::Transform,
};

pub fn spawner(
    mut commands: Commands,
    spawn_menu_selection: Res<SpawnMenuSelection>,
    mut right_mouse_events: MessageReader<MouseRightClickEvent>,
    mut spawn_sprite_event: MessageWriter<SpawnSpriteEvent>,
    player_faction: Res<PlayerFaction>,
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
            SpawnSelection::MultipleSelections => {}
            SpawnSelection::Starbase => {
                spawn_starship(
                    &mut transform,
                    &spawn_menu_selection,
                    &mut spawn_sprite_event,
                    &player_faction,
                    &mut commands,
                );
            }
        }
    });
}

fn spawn_starship(
    transform: &mut Transform,
    selected_item: &Res<'_, SpawnMenuSelection>,
    spawn_sprite_event: &mut MessageWriter<'_, SpawnSpriteEvent>,
    player_faction: &Res<PlayerFaction>,
    commands: &mut Commands<'_, '_>,
) {
    tracing::info!("starship at {:?}", transform.translation);

    if selected_item.starship_selection != StarshipType::None {
        let starship = Starship::new_from_type(
            selected_item.starship_selection,
            player_faction.player_faction,
        );

        let ship_speed = StarshipSpeed::new_from_starship_type(
            StarshipSprite::starship_type_convert_from(starship.starship_sprite),
        );

        transform.translation.z = starship.size_component.z_index;

        let entity = commands
            .spawn((
                starship,
                Selectable,
                Movement {
                    target_location: transform.translation,
                    maximum_speed: ship_speed.speed,
                    current_speed: 0.0,
                },
            ))
            .id();

        commands
            .entity(entity)
            .insert(ServerObject { id: entity.index_u32() });

        spawn_sprite_event.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starship.starship_sprite.to_string(),
            size: starship.size_component.size,
            transform: *transform,
            entity,
        }));
    }
}