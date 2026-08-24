use crate::systems::camera::spawn_camera::spawn_camera;
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
use bevy_replicon::shared::backend::ClientState;
use deadcell_solar_conquest_shared::components::space::Space;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, on_space_spawned.run_if(in_state(ClientState::Connected)));
    }
}

fn on_space_spawned(
    mut commands: Commands,
    space_query: Query<(Entity, &Space), Added<Space>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, space) in &space_query {
        println!(
            "Client received Space entity: {:?} with path: {}",
            entity, space.sprite_path
        );

        commands.entity(entity).insert(Sprite::from_image(
            asset_server.load(space.sprite_path.to_string()),
        ));
    }
}
