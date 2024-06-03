use bevy::{
    asset::AssetServer,
    color::palettes::css::{DARK_GRAY, WHITE},
    prelude::{ButtonBundle, ImageBundle, Res},
    ui::{Style, UiImage, UiRect, Val},
};

use crate::{
    components::user_interface::{SelectFacilitySpawnButton, SelectStarshipSpawnButton},
    resources::faction::{Faction, SpaceFacilityType, StarshipType},
};

pub fn create_starship_button_bundle(
    starship_type: StarshipType,
    faction: Faction,
) -> (ButtonBundle, SelectStarshipSpawnButton) {
    let icon = starship_type.icon_convert_from(faction);

    (
        ButtonBundle {
            style: button_style(),
            border_color: DARK_GRAY.into(),
            ..Default::default()
        },
        SelectStarshipSpawnButton { icon },
    )
}

pub fn create_starship_button_icon(
    asset_server: &Res<AssetServer>,
    starship_type: StarshipType,
    faction: Faction,
) -> ImageBundle {
    let icon = starship_type.icon_convert_from(faction);

    ImageBundle {
        image: UiImage::new(asset_server.load(icon.to_string())),
        background_color: WHITE.into(),
        ..Default::default()
    }
}

pub fn create_space_facility_button_bundle(
    space_facility_type: SpaceFacilityType,
    faction: Faction,
) -> (ButtonBundle, SelectFacilitySpawnButton) {
    let icon = space_facility_type.icon_convert_from(faction);

    (
        ButtonBundle {
            style: button_style(),
            border_color: DARK_GRAY.into(),
            ..Default::default()
        },
        SelectFacilitySpawnButton { icon },
    )
}

pub fn create_space_facility_button_icon(
    asset_server: &Res<AssetServer>,
    space_facility_type: SpaceFacilityType,
    faction: Faction,
) -> ImageBundle {
    let icon = space_facility_type.icon_convert_from(faction);

    ImageBundle {
        image: UiImage::new(asset_server.load(icon.to_string())),
        background_color: WHITE.into(),
        ..Default::default()
    }
}

fn button_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        border: border_style(),
        ..Default::default()
    }
}

fn border_style() -> UiRect {
    UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0))
}
