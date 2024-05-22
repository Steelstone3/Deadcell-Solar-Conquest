use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        event::EventReader,
        system::{Commands, Res, ResMut},
    },
    math::Vec2,
    sprite::{Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
};

use crate::{
    components::animation_timer::AnimationTimer,
    events::spawn_animated_sprite_event::SpawnAnimatedSpriteEvent,
};

pub fn spawn_animated_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_animated_sprite_events: EventReader<SpawnAnimatedSpriteEvent>,
) {
    for spawn_animated_sprite_event in spawn_animated_sprite_events.read() {
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(
                spawn_animated_sprite_event.sprite_tile_size,
                spawn_animated_sprite_event.sprite_tile_size,
            ),
            spawn_animated_sprite_event.frame_count,
            1,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        if let Some(mut entity) =
            commands.get_entity(spawn_animated_sprite_event.spawn_sprite_event.entity)
        {
            entity.insert((
                SpriteSheetBundle {
                    sprite: Sprite {
                        custom_size: Some(spawn_animated_sprite_event.spawn_sprite_event.size),
                        ..Default::default()
                    },
                    texture: asset_server.load(
                        spawn_animated_sprite_event
                            .spawn_sprite_event
                            .sprite_path
                            .to_string(),
                    ),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    },
                    transform: spawn_animated_sprite_event.spawn_sprite_event.transform,
                    ..Default::default()
                },
                AnimationTimer::new(
                    spawn_animated_sprite_event.frame_timing,
                    spawn_animated_sprite_event.frame_count,
                ),
            ));
        }
    }
}
