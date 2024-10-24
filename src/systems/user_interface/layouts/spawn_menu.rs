use bevy::prelude::{Res, ResMut};
use bevy_egui::{egui, EguiContexts};

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
        SpawnSelection::MultiSelection => {}
        SpawnSelection::StarshipConstructionYard => {
            // TODO AH Creates a window in a window this should move to using the exisiting window
            egui::Window::new("Spawn Menu").show(contexts.ctx_mut(), |ui| {
                ui.label("Starship Construction Yard");

                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Fighter")).clicked() {
                    let selection = StarshipType::Fighter;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }

                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Torpedo Ship")).clicked() {
                    let selection = StarshipType::TorpedoShip;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }

                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Bomber")).clicked() {
                    let selection = StarshipType::Bomber;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }

                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Frigate")).clicked() {
                    let selection = StarshipType::Frigate;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }

                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Battle Cruiser")).clicked() {
                    let selection = StarshipType::BattleCruiser;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }

                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Dreadnought")).clicked() {
                    let selection = StarshipType::Dreadnought;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }
            });
        }
        SpawnSelection::SupportShip => {
            // TODO AH Creates a window in a window this should move to using the exisiting window
            egui::Window::new("Spawn Menu").show(contexts.ctx_mut(), |ui| {
                ui.label("Support Ship");

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
        }
        SpawnSelection::Starbase => {
            // TODO AH Creates a window in a window this should move to using the exisiting window
            egui::Window::new("Spawn Menu").show(contexts.ctx_mut(), |ui| {
                ui.label("Starbase");

                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Support Ship")).clicked() {
                    let selection = StarshipType::SupportShip;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }
                // TODO AH Add images for the buttons and size to TILE_SIZE * 2.0
                if ui.add(egui::Button::new("Scout")).clicked() {
                    let selection = StarshipType::Scout;
                    spawn_menu_selection.starship_selection =
                        selection.icon_convert_from(player_faction.player_faction);
                }
            });
        }
    }
}
