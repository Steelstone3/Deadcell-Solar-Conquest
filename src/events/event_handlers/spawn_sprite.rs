use crate::{
    components::animation_timer::AnimationTimer, events::spawn_sprite_event::SpawnSpriteEvent,
};
use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    math::UVec2,
    prelude::ResMut,
    sprite::{Sprite, SpriteBundle, TextureAtlas, TextureAtlasLayout},
};

pub fn spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_sprite_events: EventReader<SpawnSpriteEvent>,
) {
    for spawn_sprite_event in spawn_sprite_events.read() {
        match spawn_sprite_event.sprite_type {
            crate::events::spawn_sprite_event::SpriteType::Static => {
                if let Some(mut entity) =
                    commands.get_entity(spawn_sprite_event.spawn_sprite.entity)
                {
                    let texture = asset_server.load(&spawn_sprite_event.spawn_sprite.sprite_path);

                    entity.insert(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(spawn_sprite_event.spawn_sprite.size),
                            ..Default::default()
                        },
                        texture,
                        transform: spawn_sprite_event.spawn_sprite.transform,
                        ..Default::default()
                    });
                }
            }
            crate::events::spawn_sprite_event::SpriteType::Animated => {
                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(
                        spawn_sprite_event.spawn_animated_sprite.sprite_tile_size,
                        spawn_sprite_event.spawn_animated_sprite.sprite_tile_size,
                    ),
                    spawn_sprite_event.spawn_animated_sprite.frame_count as u32,
                    1,
                    None,
                    None,
                );
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                if let Some(mut entity) =
                    commands.get_entity(spawn_sprite_event.spawn_sprite.entity)
                {
                    entity.insert((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(spawn_sprite_event.spawn_sprite.size),
                                ..Default::default()
                            },
                            texture: asset_server
                                .load(spawn_sprite_event.spawn_sprite.sprite_path.to_string()),
                            transform: spawn_sprite_event.spawn_sprite.transform,
                            ..Default::default()
                        },
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index: 0,
                        },
                        AnimationTimer::new(
                            spawn_sprite_event.spawn_animated_sprite.frame_timing,
                            spawn_sprite_event.spawn_animated_sprite.frame_count,
                        ),
                    ));
                }
            }
        }
    }
}
