use crate::assets::images::faction_starship_sprite::{
    destruction_sprite::DestructionSprite, engine_sprite::EngineSprite,
    firing_sprite::FiringSprite, shield_sprite::ShieldSprite, starship_sprite::StarshipSprite,
};

#[allow(dead_code)]
pub struct StarshipSpriteBundle {
    pub starship_sprite: StarshipSprite,
    pub shield_sprite: ShieldSprite,
    pub engine_sprite: EngineSprite,
    pub firing_sprite: FiringSprite,
    pub destruction_sprite: DestructionSprite,
}

impl StarshipSpriteBundle {
    pub fn new(starship_sprite: StarshipSprite) -> StarshipSpriteBundle {
        match starship_sprite {
            StarshipSprite::AtarkBattleCruiser => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkBattleCruiser,
                engine_sprite: EngineSprite::AtarkBattleCruiser,
                firing_sprite: FiringSprite::AtarkBattleCruiser,
                destruction_sprite: DestructionSprite::AtarkBattleCruiser,
            },
            StarshipSprite::AtarkBomber => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkBomber,
                engine_sprite: EngineSprite::AtarkBomber,
                firing_sprite: FiringSprite::AtarkBomber,
                destruction_sprite: DestructionSprite::AtarkBomber,
            },
            StarshipSprite::AtarkDreadnought => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkDreadnought,
                engine_sprite: EngineSprite::AtarkDreadnought,
                firing_sprite: FiringSprite::AtarkDreadnought,
                destruction_sprite: DestructionSprite::AtarkDreadnought,
            },
            StarshipSprite::AtarkFighter => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkFighter,
                engine_sprite: EngineSprite::AtarkFighter,
                firing_sprite: FiringSprite::AtarkFighter,
                destruction_sprite: DestructionSprite::AtarkFighter,
            },
            StarshipSprite::AtarkFrigate => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkFrigate,
                engine_sprite: EngineSprite::AtarkFrigate,
                firing_sprite: FiringSprite::AtarkFrigate,
                destruction_sprite: DestructionSprite::AtarkFrigate,
            },
            StarshipSprite::AtarkScout => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkScout,
                engine_sprite: EngineSprite::AtarkScout,
                firing_sprite: FiringSprite::AtarkScout,
                destruction_sprite: DestructionSprite::AtarkScout,
            },
            StarshipSprite::AtarkSupportShip => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkSupportShip,
                engine_sprite: EngineSprite::AtarkSupportShip,
                firing_sprite: FiringSprite::AtarkSupportShip,
                destruction_sprite: DestructionSprite::AtarkSupportShip,
            },
            StarshipSprite::AtarkTorpedoShip => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::AtarkTorpedoShip,
                engine_sprite: EngineSprite::AtarkTorpedoShip,
                firing_sprite: FiringSprite::AtarkTorpedoShip,
                destruction_sprite: DestructionSprite::AtarkTorpedoShip,
            },
            StarshipSprite::KarcanBattleCruiser => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanBattleCruiser,
                engine_sprite: EngineSprite::KarcanBattleCruiser,
                firing_sprite: FiringSprite::KarcanBattleCruiser,
                destruction_sprite: DestructionSprite::KarcanBattleCruiser,
            },
            StarshipSprite::KarcanBomber => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanBomber,
                engine_sprite: EngineSprite::KarcanBomber,
                firing_sprite: FiringSprite::KarcanBomber,
                destruction_sprite: DestructionSprite::KarcanBomber,
            },
            StarshipSprite::KarcanDreadnought => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanDreadnought,
                engine_sprite: EngineSprite::KarcanDreadnought,
                firing_sprite: FiringSprite::KarcanDreadnought,
                destruction_sprite: DestructionSprite::KarcanDreadnought,
            },
            StarshipSprite::KarcanFighter => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanFighter,
                engine_sprite: EngineSprite::KarcanFighter,
                firing_sprite: FiringSprite::KarcanFighter,
                destruction_sprite: DestructionSprite::KarcanFighter,
            },
            StarshipSprite::KarcanFrigate => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanFrigate,
                engine_sprite: EngineSprite::KarcanFrigate,
                firing_sprite: FiringSprite::KarcanFrigate,
                destruction_sprite: DestructionSprite::KarcanFrigate,
            },
            StarshipSprite::KarcanScout => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanScout,
                engine_sprite: EngineSprite::KarcanScout,
                firing_sprite: FiringSprite::KarcanScout,
                destruction_sprite: DestructionSprite::KarcanScout,
            },
            StarshipSprite::KarcanSupportShip => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanSupportShip,
                engine_sprite: EngineSprite::KarcanSupportShip,
                firing_sprite: FiringSprite::KarcanSupportShip,
                destruction_sprite: DestructionSprite::KarcanSupportShip,
            },
            StarshipSprite::KarcanTorpedoShip => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::KarcanTorpedoShip,
                engine_sprite: EngineSprite::KarcanTorpedoShip,
                firing_sprite: FiringSprite::KarcanTorpedoShip,
                destruction_sprite: DestructionSprite::KarcanTorpedoShip,
            },
            StarshipSprite::NoozlerBattleCruiser => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerBattleCruiser,
                engine_sprite: EngineSprite::NoozlerBattleCruiser,
                firing_sprite: FiringSprite::NoozlerBattleCruiser,
                destruction_sprite: DestructionSprite::NoozlerBattleCruiser,
            },
            StarshipSprite::NoozlerBomber => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerBomber,
                engine_sprite: EngineSprite::NoozlerBomber,
                firing_sprite: FiringSprite::NoozlerBomber,
                destruction_sprite: DestructionSprite::NoozlerBomber,
            },
            StarshipSprite::NoozlerDreadnought => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerDreadnought,
                engine_sprite: EngineSprite::NoozlerDreadnought,
                firing_sprite: FiringSprite::NoozlerDreadnought,
                destruction_sprite: DestructionSprite::NoozlerDreadnought,
            },
            StarshipSprite::NoozlerFighter => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerFighter,
                engine_sprite: EngineSprite::NoozlerFighter,
                firing_sprite: FiringSprite::NoozlerFighter,
                destruction_sprite: DestructionSprite::NoozlerFighter,
            },
            StarshipSprite::NoozlerFrigate => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerFrigate,
                engine_sprite: EngineSprite::NoozlerFrigate,
                firing_sprite: FiringSprite::NoozlerFrigate,
                destruction_sprite: DestructionSprite::NoozlerFrigate,
            },
            StarshipSprite::NoozlerScout => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerScout,
                engine_sprite: EngineSprite::NoozlerScout,
                firing_sprite: FiringSprite::NoozlerScout,
                destruction_sprite: DestructionSprite::NoozlerScout,
            },
            StarshipSprite::NoozlerSupportShip => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerSupportShip,
                engine_sprite: EngineSprite::NoozlerSupportShip,
                firing_sprite: FiringSprite::NoozlerSupportShip,
                destruction_sprite: DestructionSprite::NoozlerSupportShip,
            },
            StarshipSprite::NoozlerTorpedoShip => Self {
                starship_sprite,
                shield_sprite: ShieldSprite::NoozlerTorpedoShip,
                engine_sprite: EngineSprite::NoozlerTorpedoShip,
                firing_sprite: FiringSprite::NoozlerTorpedoShip,
                destruction_sprite: DestructionSprite::NoozlerTorpedoShip,
            },
        }
    }
}
