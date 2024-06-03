use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::developer::DeveloperPlugin;

pub struct DeveloperPluginGroup;

impl PluginGroup for DeveloperPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(DeveloperPlugin)
    }
}
