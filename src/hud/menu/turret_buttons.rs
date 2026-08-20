use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    age_of_war::Age,
    game_turret::TurretType,
    hud::menu::{TurretButtons, component::menu_button},
    resource_paths,
};

pub fn turret_button_spawner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    base_age: Age,
) -> Vec<Entity> {
    let mut res = Vec::new();

    for turret_type in TurretType::iter() {
        let button = menu_button(
            commands,
            asset_server,
            TurretButtons(turret_type),
            resource_paths::load_turret_buttons(base_age, turret_type),
        );

        res.push(button);
    }

    return res;
}
