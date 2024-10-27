#[allow(clippy::enum_variant_names)]
#[derive(Default, PartialEq, Clone, Copy)]
pub enum SpawnSelection {
    #[default]
    None, // Mothing selected
    Other, // Anything else
    #[allow(dead_code)]
    MultipleSelections, // Use when multi-selecting
    StarshipConstructionYard, // Only
    SupportShip, // Only
    Starbase, // Only
}
