use bevy::prelude::*;
use bevy::{
    app::Plugin,
    asset::AssetServer,
    ecs::{
        entity::Entity,
        query::Added,
        system::{Commands, Query, Res},
    },
    sprite::Sprite,
};
use deadcell_solar_conquest_shared::components::space::Space;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, on_space_spawned);
    }
}

fn on_space_spawned(
    mut commands: Commands,
    space_query: Query<(Entity, &Space), Added<Space>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, space) in &space_query {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(Sprite::from_image(
                asset_server.load(&space.sprite_path.to_string()),
            ));
        }
    }
}

// app.add_observer(on_space_spawned);
// fn on_space_spawned(
//     mut commands: Commands,
//     space_queries: Query<Entity, Added<Space>>,
//     asset_server: Res<AssetServer>,
// ) {
//     for entity in &space_queries {
//         commands
//             .entity(entity)
//             .insert((Sprite::from_image(asset_server.load("space.png")),));
//     }
// }
