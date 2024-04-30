use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum CombatMusicSound {
    AstralSovereign,
    #[default]
    Pirate,
    SolarisUnion,
}

impl Display for CombatMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatMusicSound::AstralSovereign => {
                write!(formatter, "sounds/music/combat/astral_sovereign_combat.ogg")
            }
            CombatMusicSound::Pirate => {
                write!(formatter, "sounds/music/combat/pirate_combat.ogg")
            }
            CombatMusicSound::SolarisUnion => {
                write!(formatter, "sounds/music/combat/solaris_union_combat.ogg")
            }
        }
    }
}

#[cfg(test)]
mod combat_music_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/music/combat/pirate_combat.ogg";

        // When
        let file_path = CombatMusicSound::default().to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        CombatMusicSound::AstralSovereign,
        "sounds/music/combat/astral_sovereign_combat.ogg"
    )]
    #[case(CombatMusicSound::Pirate, "sounds/music/combat/pirate_combat.ogg")]
    #[case(
        CombatMusicSound::SolarisUnion,
        "sounds/music/combat/solaris_union_combat.ogg"
    )]
    fn return_the_expected_file_path(
        #[case] combat_music_sound: CombatMusicSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = combat_music_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path)
    }
}
