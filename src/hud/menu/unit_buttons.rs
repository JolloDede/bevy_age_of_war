use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    age_of_war::Age,
    game_unit::UnitType,
    hud::menu::{
        UnitButtons,
        component::{menu_button, place_holder},
    },
    resource_paths,
};

pub fn unit_button_spawner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    base_age: Age,
) -> Vec<Entity> {
    let mut res = Vec::new();

    for unit_type in UnitType::iter() {
        if unit_type == UnitType::Super {
            res.push(place_holder(commands, Some(UnitButtons(unit_type))));
            continue;
        }

        let button = menu_button(
            commands,
            asset_server,
            UnitButtons(unit_type),
            resource_paths::load_unit_buttons(base_age, unit_type),
        );

        res.push(button);
    }

    return res;
}
