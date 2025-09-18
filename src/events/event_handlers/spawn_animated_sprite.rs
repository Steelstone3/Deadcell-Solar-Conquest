use crate::{
    components::user_interface::animation_timer::AnimationTimer,
    events::spawn_animated_sprite_event::SpawnAnimatedSpriteEvent,
};
use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        event::EventReader,
        system::{Commands, Res, ResMut},
    },
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    sprite::Sprite,
};

pub fn spawn_animated_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_animated_sprite_events: EventReader<SpawnAnimatedSpriteEvent>,
) {
    for spawn_animated_sprite_event in spawn_animated_sprite_events.read() {
        if let Ok(mut entity) = commands.get_entity(spawn_animated_sprite_event.spawn_sprite.entity)
        {
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(
                    spawn_animated_sprite_event
                        .spawn_animated_sprite
                        .sprite_tile_size,
                    spawn_animated_sprite_event
                        .spawn_animated_sprite
                        .sprite_tile_size,
                ),
                spawn_animated_sprite_event
                    .spawn_animated_sprite
                    .frame_count as u32,
                1,
                None,
                None,
            );

            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let texture = asset_server.load(
                spawn_animated_sprite_event
                    .spawn_sprite
                    .sprite_path
                    .to_string(),
            );

            let mut sprite = Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            );
            sprite.custom_size = Some(spawn_animated_sprite_event.spawn_sprite.size);

            let animation_timer = AnimationTimer::new(
                spawn_animated_sprite_event
                    .spawn_animated_sprite
                    .frame_timing,
                spawn_animated_sprite_event
                    .spawn_animated_sprite
                    .frame_count,
            );

            entity.insert((
                sprite,
                animation_timer,
                spawn_animated_sprite_event.spawn_sprite.transform,
            ));
        }
    }
}
