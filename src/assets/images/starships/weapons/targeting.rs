use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen, Debug, PartialEq)]
pub enum Targeting {
    CombatTarget,
    TradingTarget,
    CombatTargetOffScreen,
    TradingTargetOffScreen,
}

impl Display for Targeting {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Targeting::CombatTarget => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_combat_target.png"
                )
            }
            Targeting::CombatTargetOffScreen => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_combat_target_off_screen_indicator.png"
                )
            }
            Targeting::TradingTarget => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_trading_target.png"
                )
            }
            Targeting::TradingTargetOffScreen => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_trading_target_off_screen_indicator.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod targeting_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        Targeting::CombatTarget,
        "images/user_interface/targetting/lock_on_combat_target.png"
    )]
    #[case(
        Targeting::CombatTargetOffScreen,
        "images/user_interface/targetting/lock_on_combat_target_off_screen_indicator.png"
    )]
    #[case(
        Targeting::TradingTarget,
        "images/user_interface/targetting/lock_on_trading_target.png"
    )]
    #[case(
        Targeting::TradingTargetOffScreen,
        "images/user_interface/targetting/lock_on_trading_target_off_screen_indicator.png"
    )]
    fn return_the_expected_file_path(
        #[case] targetting_sprite: Targeting,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = targetting_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
