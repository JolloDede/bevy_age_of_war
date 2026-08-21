use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{game::GamePlugin, hud::HudPlugin, start_screen::StartScreenPlugin, state::GameState};

pub struct AgeOfWarPlugins;

impl PluginGroup for AgeOfWarPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StartScreenPlugin::new(GameState::StartScreen))
            .add(GamePlugin::new(GameState::InGame))
            .add(HudPlugin::new(GameState::InGame))
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
