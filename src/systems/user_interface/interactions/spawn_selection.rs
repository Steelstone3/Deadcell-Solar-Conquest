#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Default, PartialEq, Clone, Copy)]
pub enum SpawnSelection {
    #[default]
    None, // Mothing selected
    Other,                    // Anything else
    MultiSelection,           // Use when multi-selecting
    StarshipConstructionYard, // Only
    SupportShip,              // Only
    Starbase,                 // Only
}
