use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarshipTypes {
    Corvette,
    Destroyer,
    Fighter,
    BattleCruiser,
    Battleship,
    TorpedoShip,
    IntelShip,
    Mothership,
    Dreadnought,
    None,
}
