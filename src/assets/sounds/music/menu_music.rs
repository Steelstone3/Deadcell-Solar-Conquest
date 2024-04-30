use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum MenuMusicSound {
    #[default]
    Menu,
    Credits,
}

impl Display for MenuMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuMusicSound::Menu => {
                write!(formatter, "sounds/music/menu/menu.ogg")
            }
            MenuMusicSound::Credits => {
                write!(formatter, "sounds/music/menu/credits.ogg")
            }
        }
    }
}

#[cfg(test)]
mod menu_music_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/music/menu/menu.ogg";

        // When
        let file_path = MenuMusicSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(MenuMusicSound::Menu, "sounds/music/menu/menu.ogg")]
    #[case(MenuMusicSound::Credits, "sounds/music/menu/credits.ogg")]
    fn return_the_expected_file_path(
        #[case] menu_music_sound: MenuMusicSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = menu_music_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
