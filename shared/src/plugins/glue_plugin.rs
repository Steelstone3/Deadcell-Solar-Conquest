// shared.rs (Imported by both client and server targets)
use crate::components::space::Space;
use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Space>();
        app.replicate::<Transform>();
    }
}
