use crate::resources::server_address::ServerAddress;
use crate::systems::client::client_connection::create_client_transport_configuration;
use crate::{
    components::spawn_selection::SpawnSelection,
    resources::spawn_menu_selection::SpawnMenuSelection,
};
use bevy::ecs::system::{Commands, Res};
use bevy::prelude::ResMut;
use bevy_egui::{EguiContexts, egui};
use bevy_replicon_renet::RenetClient;
use bevy_replicon_renet::renet::DefaultChannel;
use deadcell_solar_conquest_shared::messages::server_messages::ServerMessages;
use deadcell_solar_conquest_shared::resources::factions::{Factions, PlayerFaction};
use deadcell_solar_conquest_shared::resources::{player::Player, starship_types::StarshipTypes};

pub fn connection_menu(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut server_address: ResMut<ServerAddress>,
    player: Res<Player>,
    mut client: ResMut<RenetClient>,
) {
    match client.is_connected() {
        true => {
            match contexts.ctx_mut() {
                Ok(ctx) => {
                    egui::Window::new("Player Spawn").show(ctx, |ui| {
                        if ui.add(egui::Button::new("Spawn")).clicked() {
                            // send player connected message
                            let payload = bincode_next::serde::encode_to_vec(
                                &ServerMessages::SpawnPlayer {
                                    player: (*player).clone(),
                                },
                                bincode_next::config::standard(),
                            )
                            .unwrap_or_default();

                            client.send_message(DefaultChannel::ReliableOrdered, payload);

                            println!("Player connected")
                        }
                    });
                }
                Err(_) => println!("Spawn Player UI Error"),
            }
        }
        false => match contexts.ctx_mut() {
            Ok(ctx) => {
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
                    }
                });
            }
            Err(_) => println!("Connect To Server UI Error"),
        },
    }
}

pub fn spawn_menu(
    mut contexts: EguiContexts,
    mut spawn_menu_selection: ResMut<SpawnMenuSelection>,
    player: Res<Player>,
) {
    match spawn_menu_selection.selection {
        SpawnSelection::None => {}
        SpawnSelection::Other => {}
        SpawnSelection::MultipleSelections => {}
        SpawnSelection::Starbase => match contexts.ctx_mut() {
            Ok(ctx) => {
                egui::Window::new("Spawn Menu").show(ctx, |ui| {
                    ui.label("Starbase");

                    let items = items_from_faction(&player.faction);

                    for (label, icon) in items {
                        if ui.add(egui::Button::new(label)).clicked() {
                            let selection = icon;
                            spawn_menu_selection.starship_selection = selection;
                        }
                    }
                });
            }
            Err(_) => {
                eprintln!("Starship Menu UI Error");
            }
        },
    };
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
