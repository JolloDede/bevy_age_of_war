use bevy::{app::PluginGroupBuilder, camera::visibility::RenderLayers, prelude::*};

use crate::{consts::GAME_LAYER, game::GamePlugin, hud::HudPlugin};

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
