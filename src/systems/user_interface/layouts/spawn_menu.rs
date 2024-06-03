use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle},
        Display, GridTrack, PositionType, Style, UiImage, UiRect, Val,
    },
};

use crate::{
    assets::user_interace::icons::commander_icons::CommanderIcon,
    components::user_interface::{SpawnMenu, SpawnMenuButton},
    resources::faction::PlayerFaction,
};

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    faction: Res<PlayerFaction>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
                width: Val::Px(32.0 * 2.0),
                height: Val::Px(32.0 * 7.0 * 2.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                ..Default::default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            ..Default::default()
        })
        .insert(SpawnMenu)
        .with_children(|parent| {
            // Starships
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SpawnMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(
                                CommanderIcon::convert_from(faction.player_faction).to_string(),
                            ),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
        });
}
