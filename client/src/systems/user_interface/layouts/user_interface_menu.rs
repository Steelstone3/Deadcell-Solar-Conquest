use crate::resources::server_address::ServerAddress;
use crate::systems::client::client_connection::create_client_transport_configuration;
use crate::{
    components::spawn_selection::SpawnSelection,
    resources::spawn_menu_selection::SpawnMenuSelection,
};
use bevy::ecs::system::{Commands, Res};
use bevy::ecs::world::World;
use bevy::prelude::ResMut;
use bevy_egui::{EguiContexts, egui};
use bevy_replicon_renet::RenetClient;
use bevy_replicon_renet::netcode::{NetcodeClientTransport, NetcodeError};
use deadcell_solar_conquest_shared::resources::factions::{Factions, PlayerFaction};
use deadcell_solar_conquest_shared::resources::{player::Player, starship_types::StarshipTypes};

pub fn user_interface_menu(
    mut commands: Commands,
    mut contexts: EguiContexts,
    // mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    mut player: ResMut<Player>,
    mut server_address: ResMut<ServerAddress>,
    mut client: Res<RenetClient>,
) {
    match contexts.ctx_mut() {
        Ok(ctx) => match !client.is_connected() {
            true => {
                egui::Window::new("Client Connection Menu").show(ctx, |ui| {
                    ui.label("Server Address");
                    ui.text_edit_singleline(&mut server_address.server_address);

                    if ui.add(egui::Button::new("Connect")).clicked() {
                        let transport = match create_client_transport_configuration(
                            &server_address.server_address,
                        ) {
                            Ok(transport) => transport,
                            Err(_) => {
                                println!("Failed to create client transport configuration");
                                return;
                            }
                        };

                        commands.insert_resource(transport);

                        match client.is_connected() {
                            true => println!("Player connected"),
                            false => println!("Player connection failed"),
                        }
                    }
                });
            }
            false => {
                eprintln!("Player disconnected")
            }
        },
        Err(_) => eprintln!("Failed to load user interface"),
    }
}

// pub fn client_connection_menu(mut contexts: EguiContexts, mut player: ResMut<Player>) {
//     if let Ok(ctx) = contexts.ctx_mut() {
//         egui::Window::new("Spawn Menu").show(ctx, |ui| {
//             ui.label("Starbase");

//             let items = vec![
//                 ("Corvette", StarshipTypes::Corvette),
//                 ("Destroyer", StarshipTypes::Destroyer),
//                 ("Fighter", StarshipTypes::Fighter),
//             ];

//             for (label, icon) in items {
//                 if ui.add(egui::Button::new(label)).clicked() {
//                     println!("hi");
//                 }
//             }
//         });
//     } else {
//         eprintln!("Starship failed to render");
//     }
// }

// TODO implent client connection through a "player" resource which will have connection status, id and faction

#[allow(dead_code)]
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

fn items_from_faction(player_faction: &PlayerFaction) -> Vec<(&str, StarshipTypes)> {
    match player_faction.faction {
        Factions::GranokImperialEmpire => vec![
            ("Corvette", StarshipTypes::Corvette),
            ("Destroyer", StarshipTypes::Destroyer),
            ("Fighter", StarshipTypes::Fighter),
        ],
        Factions::StarGuardAlliance => vec![
            ("Battle Cruiser", StarshipTypes::BattleCruiser),
            ("Battleship", StarshipTypes::Battleship),
            ("Corvette", StarshipTypes::Corvette),
            ("Destroyer", StarshipTypes::Destroyer),
            ("Torpedo Ship", StarshipTypes::TorpedoShip),
        ],
        Factions::UniversalMechanicalContingent => {
            vec![
                ("Destroyer", StarshipTypes::Destroyer),
                ("Intel Ship", StarshipTypes::IntelShip),
            ]
        }
        Factions::VoidwalkerCollective => {
            vec![
                ("Dreadnought", StarshipTypes::Dreadnought),
                ("Fighter", StarshipTypes::Fighter),
            ]
        }
        Factions::None => vec![],
    }
}
