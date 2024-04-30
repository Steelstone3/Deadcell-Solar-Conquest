use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum ExplorationMusicSound {
    AlienContact,
    #[default]
    DeepSpace1,
    DeepSpace2,
    DeepSpace3,
    NebulaExploration1,
    NebulaExploration2,
    NebulaExplorationScary,
}

impl Display for ExplorationMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplorationMusicSound::AlienContact => {
                write!(formatter, "sounds/music/exploration/alien_contact.ogg")
            }
            ExplorationMusicSound::DeepSpace1 => {
                write!(formatter, "sounds/music/exploration/deep_space_1.ogg")
            }
            ExplorationMusicSound::DeepSpace2 => {
                write!(formatter, "sounds/music/exploration/deep_space_2.ogg")
            }
            ExplorationMusicSound::DeepSpace3 => {
                write!(formatter, "sounds/music/exploration/deep_space_3.ogg")
            }
            ExplorationMusicSound::NebulaExploration1 => {
                write!(
                    formatter,
                    "sounds/music/exploration/nebula_exploration_1.ogg"
                )
            }
            ExplorationMusicSound::NebulaExploration2 => {
                write!(
                    formatter,
                    "sounds/music/exploration/nebula_exploration_2.ogg"
                )
            }
            ExplorationMusicSound::NebulaExplorationScary => {
                write!(
                    formatter,
                    "sounds/music/exploration/nebula_exploration_scary.ogg"
                )
            }
        }
    }
}

#[cfg(test)]
mod exploration_music_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/music/exploration/deep_space_1.ogg";

        // When
        let file_path = ExplorationMusicSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        ExplorationMusicSound::AlienContact,
        "sounds/music/exploration/alien_contact.ogg"
    )]
    #[case(
        ExplorationMusicSound::DeepSpace1,
        "sounds/music/exploration/deep_space_1.ogg"
    )]
    #[case(
        ExplorationMusicSound::DeepSpace2,
        "sounds/music/exploration/deep_space_2.ogg"
    )]
    #[case(
        ExplorationMusicSound::DeepSpace3,
        "sounds/music/exploration/deep_space_3.ogg"
    )]
    #[case(
        ExplorationMusicSound::NebulaExploration1,
        "sounds/music/exploration/nebula_exploration_1.ogg"
    )]
    #[case(
        ExplorationMusicSound::NebulaExploration2,
        "sounds/music/exploration/nebula_exploration_2.ogg"
    )]
    #[case(
        ExplorationMusicSound::NebulaExplorationScary,
        "sounds/music/exploration/nebula_exploration_scary.ogg"
    )]
    fn return_the_expected_file_path(
        #[case] exploration_music_sound: ExplorationMusicSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = exploration_music_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
