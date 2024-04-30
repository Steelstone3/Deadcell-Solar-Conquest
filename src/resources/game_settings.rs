use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GameSettings {
    pub number_of_players: i8,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            number_of_players: 4,
        }
    }
}
