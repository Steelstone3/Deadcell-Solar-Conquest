use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StarbaseTypes {
    Mothership,
    Starbase,
    None,
}
