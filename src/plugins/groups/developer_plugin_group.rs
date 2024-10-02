use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::developer::DeveloperPlugin;

use super::user_interface_plugin::UserInterfacePlugin;

pub struct DeveloperPluginGroup;

impl PluginGroup for DeveloperPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(DeveloperPlugin)
        .add(UserInterfacePlugin)
    }
}
