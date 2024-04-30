use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SelectionSprite {
    TeamSelect1,
}

impl Display for SelectionSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionSprite::TeamSelect1 => {
                write!(formatter, "images/selection/Team_Select-1.png")
            }
        }
    }
}

#[cfg(test)]
mod targeting_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(SelectionSprite::TeamSelect1, "images/selection/Team_Select-1.png")]

    fn return_the_expected_file_path(
        #[case] selection_sprite: SelectionSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = selection_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
