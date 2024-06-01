use crate::assets::images::faction_starships::starships::StarshipSprite;

pub enum Faction {
    Atark,
    Karcan,
    Noozler,
}

impl Faction {
    pub fn new(starship_sprite: StarshipSprite) -> Faction {
        match starship_sprite {
            StarshipSprite::AtarkBattleCruiser => Faction::Atark,
            StarshipSprite::AtarkBomber => Faction::Atark,
            StarshipSprite::AtarkDreadnought => Faction::Atark,
            StarshipSprite::AtarkFighter => Faction::Atark,
            StarshipSprite::AtarkFrigate => Faction::Atark,
            StarshipSprite::AtarkScout => Faction::Atark,
            StarshipSprite::AtarkSupportShip => Faction::Atark,
            StarshipSprite::AtarkTorpedoShip => Faction::Atark,
            StarshipSprite::KarcanBattleCruiser => Faction::Karcan,
            StarshipSprite::KarcanBomber => Faction::Karcan,
            StarshipSprite::KarcanDreadnought => Faction::Karcan,
            StarshipSprite::KarcanFighter => Faction::Karcan,
            StarshipSprite::KarcanFrigate => Faction::Karcan,
            StarshipSprite::KarcanScout => Faction::Karcan,
            StarshipSprite::KarcanSupportShip => Faction::Karcan,
            StarshipSprite::KarcanTorpedoShip => Faction::Karcan,
            StarshipSprite::NoozlerBattleCruiser => Faction::Noozler,
            StarshipSprite::NoozlerBomber => Faction::Noozler,
            StarshipSprite::NoozlerDreadnought => Faction::Noozler,
            StarshipSprite::NoozlerFighter => Faction::Noozler,
            StarshipSprite::NoozlerFrigate => Faction::Noozler,
            StarshipSprite::NoozlerScout => Faction::Noozler,
            StarshipSprite::NoozlerSupportShip => Faction::Noozler,
            StarshipSprite::NoozlerTorpedoShip => Faction::Noozler,
        }
    }
}
