use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    ui::{node_bundles::NodeBundle, Display, GridTrack, PositionType, Style, Val},
};

use crate::{
    components::user_interface::SpawnSubMenuButton,
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SpawnSubMenuQuery,
    resources::{
        faction::{PlayerFaction, SpaceFacilityType, StarshipType},
        spawn_menu_selection::SpawnMenuSelection,
    },
    systems::user_interface::{
        interactions::spawn_selection::SpawnSelection,
        styles::{
            create_space_facility_button_bundle, create_space_facility_button_icon,
            create_starship_button_bundle, create_starship_button_icon,
        },
    },
};

use super::despawn_spawn_sub_menu::despawn_sub_menus;

pub fn spawn_sub_menu(
    selected_item: Res<SpawnMenuSelection>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SpawnSubMenuQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    faction: Res<PlayerFaction>,
) {
    match selected_item.selection {
        SpawnSelection::None => {
            despawn_sub_menus(
                selected_item,
                user_interface_events,
                sub_menu_queries,
                commands,
            );
        }
        SpawnSelection::Other => {
            despawn_sub_menus(
                selected_item,
                user_interface_events,
                sub_menu_queries,
                commands,
            );
        }
        SpawnSelection::MultiSelection => {
            despawn_sub_menus(
                selected_item,
                user_interface_events,
                sub_menu_queries,
                commands,
            );
        }
        SpawnSelection::StarshipConstructionYard => {
            for _ in user_interface_events.read() {
                // Remove UI
                if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
                    commands.entity(sub_menu_query.entity).despawn_recursive();
                }

                // Spawn UI
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                            grid_template_rows: vec![
                                GridTrack::flex(1.0),
                                GridTrack::flex(1.0),
                                GridTrack::flex(1.0),
                            ],
                            width: Val::Px(64.0 * 2.0),
                            height: Val::Px(64.0 * 3.0),
                            position_type: PositionType::Absolute,
                            left: Val::Px(64.0),
                            top: Val::Percent(0.0),
                            ..Default::default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                        ..Default::default()
                    })
                    .insert(SpawnSubMenuButton)
                    // Fighter
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::Fighter,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::Fighter,
                                    faction.player_faction,
                                ));
                            });
                    })
                    // Torpedo Ship
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::TorpedoShip,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::TorpedoShip,
                                    faction.player_faction,
                                ));
                            });
                    })
                    // Bomber
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::Bomber,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::Bomber,
                                    faction.player_faction,
                                ));
                            });
                    })
                    // Frigate
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::Frigate,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::Frigate,
                                    faction.player_faction,
                                ));
                            });
                    })
                    // Battlecruiser
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::BattleCruiser,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::BattleCruiser,
                                    faction.player_faction,
                                ));
                            });
                    })
                    // Dreadnought
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::Dreadnought,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::Dreadnought,
                                    faction.player_faction,
                                ));
                            });
                    });
            }
        }
        SpawnSelection::SupportShip => {
            for _ in user_interface_events.read() {
                // Remove UI
                if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
                    commands.entity(sub_menu_query.entity).despawn_recursive();
                }

                // Spawn UI
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                            grid_template_rows: vec![GridTrack::flex(1.0)],
                            width: Val::Px(64.0 * 2.0),
                            height: Val::Px(64.0 * 1.0),
                            position_type: PositionType::Absolute,
                            left: Val::Px(64.0),
                            top: Val::Percent(0.0),
                            ..Default::default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                        ..Default::default()
                    })
                    .insert(SpawnSubMenuButton)
                    // Starship Construction Yard
                    .with_children(|parent| {
                        parent
                            .spawn(create_space_facility_button_bundle(
                                SpaceFacilityType::SpaceShipConstructionYard,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_space_facility_button_icon(
                                    &asset_server,
                                    SpaceFacilityType::SpaceShipConstructionYard,
                                    faction.player_faction,
                                ));
                            });
                    });
            }
        }
        SpawnSelection::Starbase => {
            for _ in user_interface_events.read() {
                // Remove UI
                if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
                    commands.entity(sub_menu_query.entity).despawn_recursive();
                }

                // Spawn UI
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                            grid_template_rows: vec![GridTrack::flex(1.0)],
                            width: Val::Px(64.0 * 2.0),
                            height: Val::Px(64.0 * 1.0),
                            position_type: PositionType::Absolute,
                            left: Val::Px(64.0),
                            top: Val::Percent(0.0),
                            ..Default::default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                        ..Default::default()
                    })
                    .insert(SpawnSubMenuButton)
                    // Support Ship
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::SupportShip,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::SupportShip,
                                    faction.player_faction,
                                ));
                            });
                    })
                    // Scout
                    .with_children(|parent| {
                        parent
                            .spawn(create_starship_button_bundle(
                                StarshipType::Scout,
                                faction.player_faction,
                            ))
                            .with_children(|parent| {
                                parent.spawn(create_starship_button_icon(
                                    &asset_server,
                                    StarshipType::Scout,
                                    faction.player_faction,
                                ));
                            });
                    });
            }
        }
    }
}
