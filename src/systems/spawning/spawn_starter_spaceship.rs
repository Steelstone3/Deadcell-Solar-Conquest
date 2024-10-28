use crate::{
    assets::images::{
        faction_starship_sprite::starship_sprite::StarshipSprite, starship_type::StarshipType,
    },
    components::{
        faction::starship::{Starship, StarshipSpeed},
        server::server_object::ServerObject,
        user_interface::{controllable::Movement, selection::Selectable},
    },
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    queries::faction_queries::SpaceStationQuery,
    resources::faction::PlayerFaction,
};
use bevy::prelude::{Commands, EventWriter, Query, Res};

pub fn spawn_starter_spaceship(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    space_station_queries: Query<SpaceStationQuery>,
    player_faction: Res<PlayerFaction>,
) {
    for space_station_query in space_station_queries.iter() {
        let starship = Starship::new(
            StarshipType::SupportShip.sprite_convert_from(player_faction.player_faction),
        );

        let mut starship_transform = *space_station_query.transform;
        starship_transform.translation.z = starship.size_component.z_index;

        let ship_speed =
            StarshipSpeed::new_from_starship_type(StarshipSprite::starship_type_convert_from(
                starship.starship_sprite_bundle.starship_sprite,
            ));

        let entity = commands
            .spawn(starship)
            .insert(Selectable)
            .insert(Movement {
                target_location: starship_transform.translation,
                maximum_speed: ship_speed.speed,
                current_speed: 0.0,
            })
            .id();

        commands
            .entity(entity)
            .insert(ServerObject { id: entity.index() });

        spawn_sprite_event.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starship.starship_sprite_bundle.starship_sprite.to_string(),
            size: starship.size_component.size,
            transform: starship_transform,
            entity,
        }));
    }
}
