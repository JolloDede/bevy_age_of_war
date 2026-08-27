use bevy::{
    log::{self, LogPlugin},
    prelude::*,
};

mod age_of_war;
use crate::age_of_war::*;

mod consts;
mod event;
mod game;
mod game_turret;
mod game_unit;
mod hud;
mod name;
mod player;
mod resource_paths;
mod start_screen;
mod state;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Age of War".to_string(),
                        resolution: (800, 400).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: log::Level::INFO,
                    ..Default::default()
                }),
        )
        .add_plugins(AgeOfWarPlugins)
        .run();
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Base {
    pub age: Age,
}

impl Base {
    pub fn new() -> Self {
        Self { age: Age::StoneAge }
    }
}
