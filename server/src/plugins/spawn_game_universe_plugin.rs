use crate::systems::spawn_universe::spawn_space::spawn_space_on_client_connected;
use bevy::
    app::{App, Plugin}
;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_space_on_client_connected);
    }
}
