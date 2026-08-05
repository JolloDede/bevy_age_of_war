use bevy::prelude::*;

mod age_of_war;
use crate::age_of_war::*;

mod consts;
mod event;
mod game;
mod game_unit;
mod hud;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Age of War".to_string(),
                resolution: (800, 400).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(AgeOfWarPlugins)
        .run();
}
