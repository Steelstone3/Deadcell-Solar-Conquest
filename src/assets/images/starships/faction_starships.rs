use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(Default, RandGen, Debug, PartialEq, Reflect)]
pub enum FactionStarshipSprite {
    AstralSovereignAllRounder,
    AstralSovereignExplorer,
    AstralSovereignFighter,
    AstralSovereignTrader,
    CosmicCommerceAssemblyAllRounder,
    CosmicCommerceAssemblyExplorer,
    CosmicCommerceAssemblyFighter,
    CosmicCommerceAssemblyTrader,
    DarkstarFaelithDominionAllRounder,
    DarkstarFaelithDominionExplorer,
    DarkstarFaelithDominionFighter,
    DarkstarFaelithDominionTrader,
    FreeStarConfederacyAllRounder,
    FreeStarConfederacyExplorer,
    FreeStarConfederacyFighter,
    FreeStarConfederacyTrader,
    LumithrinAllRounder,
    MoonhoofClanAllRounder,
    MoonhoofClanExplorer,
    MoonhoofClanFighter,
    MoonhoofClanTrader,
    #[default]
    OuterReachMiningGuildAllRounder,
    OuterReachMiningGuildExplorer,
    OuterReachMiningGuildFighter,
    OuterReachMiningGuildTrader,
    SiliconFangTechnocracyAllRounder,
    SiliconFangTechnocracyExplorer,
    SiliconFangTechnocracyFighter,
    SiliconFangTechnocracyTrader,
    SolarisUnionAllRounder,
    SolarisUnionExplorer,
    SolarisUnionFighter,
    SolarisUnionTrader,
    ZephyrarianAllRounder,
}

impl Display for FactionStarshipSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactionStarshipSprite::AstralSovereignAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/all_rounder.png"
                )
            }
            FactionStarshipSprite::AstralSovereignExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/explorer.png"
                )
            }
            FactionStarshipSprite::AstralSovereignFighter => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/fighter.png"
                )
            }
            FactionStarshipSprite::AstralSovereignTrader => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/trader.png"
                )
            }
            FactionStarshipSprite::CosmicCommerceAssemblyAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/all_rounder.png"
                )
            }
            FactionStarshipSprite::CosmicCommerceAssemblyExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/explorer.png"
                )
            }
            FactionStarshipSprite::CosmicCommerceAssemblyFighter => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/fighter.png"
                )
            }
            FactionStarshipSprite::CosmicCommerceAssemblyTrader => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/trader.png"
                )
            }
            FactionStarshipSprite::DarkstarFaelithDominionAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/all_rounder.png"
                )
            }
            FactionStarshipSprite::DarkstarFaelithDominionExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/explorer.png"
                )
            }
            FactionStarshipSprite::DarkstarFaelithDominionFighter => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/fighter.png"
                )
            }
            FactionStarshipSprite::DarkstarFaelithDominionTrader => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/trader.png"
                )
            }
            FactionStarshipSprite::FreeStarConfederacyAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/all_rounder.png"
                )
            }
            FactionStarshipSprite::FreeStarConfederacyExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/explorer.png"
                )
            }
            FactionStarshipSprite::FreeStarConfederacyFighter => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/fighter.png"
                )
            }
            FactionStarshipSprite::FreeStarConfederacyTrader => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/trader.png"
                )
            }
            FactionStarshipSprite::LumithrinAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/aliens/lumithrin_all_rounder.png"
                )
            }
            FactionStarshipSprite::MoonhoofClanAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/all_rounder.png"
                )
            }
            FactionStarshipSprite::MoonhoofClanExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/explorer.png"
                )
            }
            FactionStarshipSprite::MoonhoofClanFighter => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/fighter.png"
                )
            }
            FactionStarshipSprite::MoonhoofClanTrader => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/trader.png"
                )
            }
            FactionStarshipSprite::OuterReachMiningGuildAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/all_rounder.png"
                )
            }
            FactionStarshipSprite::OuterReachMiningGuildExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/explorer.png"
                )
            }
            FactionStarshipSprite::OuterReachMiningGuildFighter => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/fighter.png"
                )
            }
            FactionStarshipSprite::OuterReachMiningGuildTrader => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/trader.png"
                )
            }
            FactionStarshipSprite::SiliconFangTechnocracyAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/all_rounder.png"
                )
            }
            FactionStarshipSprite::SiliconFangTechnocracyExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/explorer.png"
                )
            }
            FactionStarshipSprite::SiliconFangTechnocracyFighter => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/fighter.png"
                )
            }
            FactionStarshipSprite::SiliconFangTechnocracyTrader => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/trader.png"
                )
            }
            FactionStarshipSprite::SolarisUnionAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/all_rounder.png"
                )
            }
            FactionStarshipSprite::SolarisUnionExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/explorer.png"
                )
            }
            FactionStarshipSprite::SolarisUnionFighter => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/fighter.png"
                )
            }
            FactionStarshipSprite::SolarisUnionTrader => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/trader.png"
                )
            }
            FactionStarshipSprite::ZephyrarianAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/aliens/zephyrarian_all_rounder.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod faction_starships_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path =
            "images/starships/factions/outer_reach_mining_guild/all_rounder.png";
        let faction_starship = FactionStarshipSprite::default();

        // When
        let file_path = faction_starship.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        FactionStarshipSprite::AstralSovereignAllRounder,
        "images/starships/factions/astral_sovereign/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::AstralSovereignExplorer,
        "images/starships/factions/astral_sovereign/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::AstralSovereignFighter,
        "images/starships/factions/astral_sovereign/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::AstralSovereignTrader,
        "images/starships/factions/astral_sovereign/trader.png"
    )]
    #[case(
        FactionStarshipSprite::CosmicCommerceAssemblyAllRounder,
        "images/starships/factions/cosmic_commerce_assembly/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::CosmicCommerceAssemblyExplorer,
        "images/starships/factions/cosmic_commerce_assembly/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::CosmicCommerceAssemblyFighter,
        "images/starships/factions/cosmic_commerce_assembly/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::CosmicCommerceAssemblyTrader,
        "images/starships/factions/cosmic_commerce_assembly/trader.png"
    )]
    #[case(
        FactionStarshipSprite::DarkstarFaelithDominionAllRounder,
        "images/starships/factions/darkstar_faelith_dominion/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::DarkstarFaelithDominionExplorer,
        "images/starships/factions/darkstar_faelith_dominion/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::DarkstarFaelithDominionFighter,
        "images/starships/factions/darkstar_faelith_dominion/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::DarkstarFaelithDominionTrader,
        "images/starships/factions/darkstar_faelith_dominion/trader.png"
    )]
    #[case(
        FactionStarshipSprite::FreeStarConfederacyAllRounder,
        "images/starships/factions/free_star_confederacy/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::FreeStarConfederacyExplorer,
        "images/starships/factions/free_star_confederacy/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::FreeStarConfederacyFighter,
        "images/starships/factions/free_star_confederacy/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::FreeStarConfederacyTrader,
        "images/starships/factions/free_star_confederacy/trader.png"
    )]
    #[case(
        FactionStarshipSprite::LumithrinAllRounder,
        "images/starships/factions/aliens/lumithrin_all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::MoonhoofClanAllRounder,
        "images/starships/factions/moonhoof_clan/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::MoonhoofClanExplorer,
        "images/starships/factions/moonhoof_clan/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::MoonhoofClanFighter,
        "images/starships/factions/moonhoof_clan/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::MoonhoofClanTrader,
        "images/starships/factions/moonhoof_clan/trader.png"
    )]
    #[case(
        FactionStarshipSprite::OuterReachMiningGuildAllRounder,
        "images/starships/factions/outer_reach_mining_guild/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::OuterReachMiningGuildExplorer,
        "images/starships/factions/outer_reach_mining_guild/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::OuterReachMiningGuildFighter,
        "images/starships/factions/outer_reach_mining_guild/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::OuterReachMiningGuildTrader,
        "images/starships/factions/outer_reach_mining_guild/trader.png"
    )]
    #[case(
        FactionStarshipSprite::SiliconFangTechnocracyAllRounder,
        "images/starships/factions/silicon_fang_technocracy/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::SiliconFangTechnocracyExplorer,
        "images/starships/factions/silicon_fang_technocracy/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::SiliconFangTechnocracyFighter,
        "images/starships/factions/silicon_fang_technocracy/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::SiliconFangTechnocracyTrader,
        "images/starships/factions/silicon_fang_technocracy/trader.png"
    )]
    #[case(
        FactionStarshipSprite::SolarisUnionAllRounder,
        "images/starships/factions/solaris_union/all_rounder.png"
    )]
    #[case(
        FactionStarshipSprite::SolarisUnionExplorer,
        "images/starships/factions/solaris_union/explorer.png"
    )]
    #[case(
        FactionStarshipSprite::SolarisUnionFighter,
        "images/starships/factions/solaris_union/fighter.png"
    )]
    #[case(
        FactionStarshipSprite::SolarisUnionTrader,
        "images/starships/factions/solaris_union/trader.png"
    )]
    #[case(
        FactionStarshipSprite::ZephyrarianAllRounder,
        "images/starships/factions/aliens/zephyrarian_all_rounder.png"
    )]
    fn return_the_expected_file_path(
        #[case] faction_starship: FactionStarshipSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = faction_starship.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
