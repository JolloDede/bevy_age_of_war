use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{game::GamePlugin, hud::HudPlugin};

pub struct AgeOfWarPlugins;

impl PluginGroup for AgeOfWarPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GamePlugin)
            .add(HudPlugin)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Age {
    StoneAge,
    Medival,
    Renaissance,
    Modern,
    Future,
}
