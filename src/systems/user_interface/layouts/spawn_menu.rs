use bevy::prelude::{Res, ResMut};
use bevy_egui::{EguiContexts, egui};

use crate::{
    assets::images::starship_sprite::StarshipType,
    resources::{faction::PlayerFaction, spawn_menu_selection::SpawnMenuSelection},
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

pub fn spawn_menu(
    mut contexts: EguiContexts,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    player_faction: Res<PlayerFaction>,
) {
    match spawn_menu_selection.selection {
        SpawnSelection::None => {}
        SpawnSelection::Other => {}
        SpawnSelection::MultipleSelections => {}
        SpawnSelection::Starbase => {
            if let Ok(ctx) = contexts.ctx_mut() {
                egui::Window::new("Spawn Menu").show(ctx, |ui| {
                    ui.label("Starbase");

                    let items = items_from_faction(&player_faction);

                    for (label, icon) in items {
                        if ui.add(egui::Button::new(label)).clicked() {
                            let selection = icon;
                            spawn_menu_selection.starship_selection =
                                selection;
                        }
                    }
                });
            } else {
                eprintln!("Starship failed to render");
            }
        }
    }
}

fn items_from_faction(player_faction: &PlayerFaction) -> Vec<(&str, StarshipType)> {
    match player_faction.player_faction {
        crate::resources::faction::Faction::GranokImperialEmpire => vec![
            ("Corvette", StarshipType::Corvette),
            ("Destroyer", StarshipType::Destroyer),
            ("Fighter", StarshipType::Fighter),
        ],
        crate::resources::faction::Faction::StarGuardAlliance => vec![
            ("Battle Cruiser", StarshipType::BattleCruiser),
            ("Battleship", StarshipType::Battleship),
            ("Corvette", StarshipType::Corvette),
            ("Destroyer", StarshipType::Destroyer),
            ("Torpedo Ship", StarshipType::TorpedoShip),
        ],
        crate::resources::faction::Faction::UniversalMechanicalContigent => {
            vec![
                ("Destroyer", StarshipType::Destroyer),
                ("Intel Ship", StarshipType::IntelShip),
            ]
        }
        crate::resources::faction::Faction::VoidwalkerCollective => {
            vec![
                ("Dreadnought", StarshipType::Dreadnought),
                ("Fighter", StarshipType::Fighter),
            ]
        }
        crate::resources::faction::Faction::None => vec![],
    }
}
