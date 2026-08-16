use bevy::prelude::{Res, ResMut};
use bevy_egui::{EguiContexts, egui};
use deadcell_solar_conquest_shared::{
    assets::images::starship_sprite::StarshipType,
    components::client::user_interface::spawn_selection::SpawnSelection,
    resources::{
        faction::{
            Faction::{
                self, GranokImperialEmpire, StarGuardAlliance, UniversalMechanicalContigent,
                VoidwalkerCollective,
            },
            PlayerFaction,
        },
        spawn_menu_selection::SpawnMenuSelection,
    },
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
                            spawn_menu_selection.starship_selection = selection;
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
        GranokImperialEmpire => vec![
            ("Corvette", StarshipType::Corvette),
            ("Destroyer", StarshipType::Destroyer),
            ("Fighter", StarshipType::Fighter),
        ],
        StarGuardAlliance => vec![
            ("Battle Cruiser", StarshipType::BattleCruiser),
            ("Battleship", StarshipType::Battleship),
            ("Corvette", StarshipType::Corvette),
            ("Destroyer", StarshipType::Destroyer),
            ("Torpedo Ship", StarshipType::TorpedoShip),
        ],
        UniversalMechanicalContigent => {
            vec![
                ("Destroyer", StarshipType::Destroyer),
                ("Intel Ship", StarshipType::IntelShip),
            ]
        }
        VoidwalkerCollective => {
            vec![
                ("Dreadnought", StarshipType::Dreadnought),
                ("Fighter", StarshipType::Fighter),
            ]
        }
        Faction::None => vec![],
    }
}
