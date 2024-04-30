use bevy::reflect::Reflect;
use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq, Eq, Reflect)]
pub enum ImpactSound {
    #[default]
    Impact1,
    Impact2,
    Impact3,
    Impact4,
    Impact5,
    Impact6,
    Impact7,
    Impact8,
    Impact9,
    Impact10,
    Impact11,
    Impact12,
}

impl Display for ImpactSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImpactSound::Impact1 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_1.ogg")
            }
            ImpactSound::Impact2 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_2.ogg")
            }
            ImpactSound::Impact3 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_3.ogg")
            }
            ImpactSound::Impact4 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_4.ogg")
            }
            ImpactSound::Impact5 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_5.ogg")
            }
            ImpactSound::Impact6 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_6.ogg")
            }
            ImpactSound::Impact7 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_7.ogg")
            }
            ImpactSound::Impact8 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_8.ogg")
            }
            ImpactSound::Impact9 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_9.ogg")
            }
            ImpactSound::Impact10 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_10.ogg")
            }
            ImpactSound::Impact11 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_11.ogg")
            }
            ImpactSound::Impact12 => {
                write!(formatter, "sounds/starships/weapons/impacts/impact_12.ogg")
            }
        }
    }
}

#[cfg(test)]
mod impact_sound_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "sounds/starships/weapons/impacts/impact_1.ogg";
        let impact_sound = ImpactSound::default();

        // When
        let file_path = impact_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(ImpactSound::Impact1, "sounds/starships/weapons/impacts/impact_1.ogg")]
    #[case(ImpactSound::Impact2, "sounds/starships/weapons/impacts/impact_2.ogg")]
    #[case(ImpactSound::Impact3, "sounds/starships/weapons/impacts/impact_3.ogg")]
    #[case(ImpactSound::Impact4, "sounds/starships/weapons/impacts/impact_4.ogg")]
    #[case(ImpactSound::Impact5, "sounds/starships/weapons/impacts/impact_5.ogg")]
    #[case(ImpactSound::Impact6, "sounds/starships/weapons/impacts/impact_6.ogg")]
    #[case(ImpactSound::Impact7, "sounds/starships/weapons/impacts/impact_7.ogg")]
    #[case(ImpactSound::Impact8, "sounds/starships/weapons/impacts/impact_8.ogg")]
    #[case(ImpactSound::Impact9, "sounds/starships/weapons/impacts/impact_9.ogg")]
    #[case(
        ImpactSound::Impact10,
        "sounds/starships/weapons/impacts/impact_10.ogg"
    )]
    #[case(
        ImpactSound::Impact11,
        "sounds/starships/weapons/impacts/impact_11.ogg"
    )]
    #[case(
        ImpactSound::Impact12,
        "sounds/starships/weapons/impacts/impact_12.ogg"
    )]
    fn return_the_expected_file_path(
        #[case] impact_sound: ImpactSound,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = impact_sound.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
