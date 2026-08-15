use bevy::prelude::{Res, ResMut};
use bevy_egui::{EguiContexts, egui};

use crate::{
    assets::images::starship_sprite::StarshipType, resources::{faction::PlayerFaction, spawn_menu_selection::SpawnMenuSelection}, systems::user_interface::interactions::spawn_selection::SpawnSelection,
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

                    let items = [
                        ("Support Ship", StarshipType::BattleCruiser),
                        ("Scout", StarshipType::Battleship),
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
    }
}
