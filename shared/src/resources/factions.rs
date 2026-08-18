use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Factions {
    GranokImperialEmpire,
    StarGuardAlliance,
    UniversalMechanicalContigent,
    VoidwalkerCollective,
    None,
}
