use bevy::prelude::{Res, ResMut};
use bevy_egui::{EguiContexts, egui};

use crate::{
    assets::images::{space_facility_type::SpaceFacilityType, starship_type::StarshipType},
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
        SpawnSelection::StarshipConstructionYard => {
            if let Ok(ctx) = contexts.ctx_mut() {
                egui::Window::new("Spawn Menu").show(ctx, |ui| {
                    ui.label("Starship Construction Yard");

                    let items = [
                        ("Fighter", StarshipType::Fighter),
                        ("Torpedo Ship", StarshipType::TorpedoShip),
                        ("Bomber", StarshipType::Bomber),
                        ("Frigate", StarshipType::Frigate),
                        ("Battle Cruiser", StarshipType::BattleCruiser),
                        ("Dreadnought", StarshipType::Dreadnought),
                    ];

                    for (label, icon) in items {
                        if ui.add(egui::Button::new(label)).clicked() {
                            let selection = icon;
                            spawn_menu_selection.starship_selection =
                                selection.icon_convert_from(player_faction.player_faction);
                        }
                    }
                });
            } else {
                eprintln!("Starship failed to render");
            }
        }
        SpawnSelection::SupportShip => {
            if let Ok(ctx) = contexts.ctx_mut() {
                egui::Window::new("Support Ship").show(ctx, |ui| {
                    // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                    if ui
                        .add(egui::Button::new("Spaceship Construction Yard"))
                        .clicked()
                    {
                        let selection = SpaceFacilityType::SpaceShipConstructionYard;
                        spawn_menu_selection.space_facility_selection =
                            selection.icon_convert_from(player_faction.player_faction);
                    }
                });
            } else {
                eprintln!("Starship failed to render");
            }

            // // TODO AH Creates a window in a window this should move to using the exisiting window
            // egui::Window::new("Spawn Menu").show(contexts.ctx_mut(), |ui| {
            //     ui.label("Support Ship");

            //     // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
            //     if ui
            //         .add(egui::Button::new("Spaceship Construction Yard"))
            //         .clicked()
            //     {
            //         let selection = SpaceFacilityType::SpaceShipConstructionYard;
            //         spawn_menu_selection.space_facility_selection =
            //             selection.icon_convert_from(player_faction.player_faction);
            //     }
            // });
        }
        SpawnSelection::Starbase => {
            if let Ok(ctx) = contexts.ctx_mut() {
                egui::Window::new("Spawn Menu").show(ctx, |ui| {
                    ui.label("Starbase");

                    let items = [
                        ("Support Ship", StarshipType::SupportShip),
                        ("Scout", StarshipType::Scout),
                    ];

                    for (label, icon) in items {
                        if ui.add(egui::Button::new(label)).clicked() {
                            let selection = icon;
                            spawn_menu_selection.starship_selection =
                                selection.icon_convert_from(player_faction.player_faction);
                        }
                    }
                });
            } else {
                eprintln!("Starship failed to render");
            }

            // // TODO AH Creates a window in a window this should move to using the exisiting window
            // egui::Window::new("Spawn Menu").show(contexts.ctx_mut(), |ui| {
            //     ui.label("Starbase");

            //     // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
            //     if ui.add(egui::Button::new("Support Ship")).clicked() {
            //         let selection = StarshipType::SupportShip;
            //         spawn_menu_selection.starship_selection =
            //             selection.icon_convert_from(player_faction.player_faction);
            //     }
            //     // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
            //     if ui.add(egui::Button::new("Scout")).clicked() {
            //         let selection = StarshipType::Scout;
            //         spawn_menu_selection.starship_selection =
            //             selection.icon_convert_from(player_faction.player_faction);
            //     }
            // });
        }
    }
}
