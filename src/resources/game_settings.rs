use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GameSettings {
    pub number_of_players: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            number_of_players: 2,
        }
    }
}
