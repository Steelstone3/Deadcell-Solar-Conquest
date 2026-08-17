use crate::{
    assets::images::starship_sprite::{StarbaseSprite, StarbaseType, StarshipType},
    components::{
        faction::{starbase::Starbase, starship::StarshipSpeed},
        user_interface::{controllable::Movement, selection::Selectable},
    },
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    resources::{faction::PlayerFaction, game_settings::GameSettings},
};
use bevy::{
    ecs::{message::MessageWriter, system::Commands},
    math::Quat,
    prelude::Res,
    transform::components::Transform,
};
use rand::Rng;

pub fn spawn_starter_starbase(
    mut commands: Commands,
    mut spawn_sprite_event: MessageWriter<SpawnSpriteEvent>,
    player_faction: Res<PlayerFaction>,
    settings: Res<GameSettings>,
) {
    let mut rng = rand::thread_rng();
    let angle = 360.0 / rng.gen_range(1.0..4.0) as f32;

    let starbase_sprite = StarbaseSprite::sprite_convert_from(player_faction.player_faction);

    let starbase = Starbase::new(starbase_sprite);

    let x: f32 = rand::thread_rng().gen_range(0.0..settings.map_size as f32)
        + starbase.size_component.size.x * 1.5;
    let y: f32 = rand::thread_rng().gen_range(0.0..settings.map_size as f32)
        + starbase.size_component.size.x * 1.5;

    let transform = Transform::from_xyz(x, y, starbase.size_component.z_index)
        .with_rotation(Quat::from_rotation_z(angle.to_radians()));

    let starbase_type = StarbaseType::starbase_type_convert_from(starbase_sprite);

    if starbase_type == StarbaseType::Mothership {
        spawn_sprite_event.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starbase.sprite_path.to_string(),
            size: starbase.size_component.size,
            transform,
            entity: commands
                .spawn(starbase)
                .insert(Selectable)
                .insert(transform)
                .insert(Movement {
                    target_location: transform.translation,
                    maximum_speed: StarshipSpeed::new_from_starship_type(StarshipType::Mothership)
                        .speed,
                    current_speed: 0.0,
                })
                .id(),
        }));
    } else {
        spawn_sprite_event.write(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starbase.sprite_path.to_string(),
            size: starbase.size_component.size,
            transform,
            entity: commands
                .spawn(starbase)
                .insert(Selectable)
                .insert(transform)
                .id(),
        }));
    }
}
