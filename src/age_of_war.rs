use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::hud::HudPlugin;

pub struct AgeOfWarPlugins;

impl PluginGroup for AgeOfWarPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GamePlugin)
            .add(HudPlugin)
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bases);
    }
}

#[derive(Component)]
struct Base;

const BASE_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
fn spawn_bases(mut commands: Commands) {
    // commands
    //     .spawn(Sprite::from_color(BASE_COLOR, Vec2::new(10.0, 10.0)))
    //     .insert(Base);
    // commands
    //     .spawn(Sprite::from_color(BASE_COLOR, Vec2::new(10.0, 10.0)))
    //     .insert(Base);
}
