use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum TradingMusicSound {
    #[default]
    Trading1,
    Trading2,
    Trading3,
    Trading4,
    Trading5,
}

impl Display for TradingMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradingMusicSound::Trading1 => {
                write!(formatter, "sounds/music/trading/trading_1.ogg")
            }
            TradingMusicSound::Trading2 => {
                write!(formatter, "sounds/music/trading/trading_2.ogg")
            }
            TradingMusicSound::Trading3 => {
                write!(formatter, "sounds/music/trading/trading_3.ogg")
            }
            TradingMusicSound::Trading4 => {
                write!(formatter, "sounds/music/trading/trading_4.ogg")
            }
            TradingMusicSound::Trading5 => {
                write!(formatter, "sounds/music/trading/trading_5.ogg")
            }
        }
    }
}

#[cfg(test)]
mod trading_music_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/music/trading/trading_1.ogg";

        // When
        let file_path = TradingMusicSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(TradingMusicSound::Trading1, "sounds/music/trading/trading_1.ogg")]
    #[case(TradingMusicSound::Trading2, "sounds/music/trading/trading_2.ogg")]
    #[case(TradingMusicSound::Trading3, "sounds/music/trading/trading_3.ogg")]
    #[case(TradingMusicSound::Trading4, "sounds/music/trading/trading_4.ogg")]
    #[case(TradingMusicSound::Trading5, "sounds/music/trading/trading_5.ogg")]
    fn return_the_expected_file_path(
        #[case] trading_music_sound: TradingMusicSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = trading_music_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
