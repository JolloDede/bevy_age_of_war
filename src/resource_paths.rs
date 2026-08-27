use bevy::log::debug;

use crate::{
    age_of_war::Age,
    game_turret::TurretType,
    game_unit::UnitType,
    hud::{MenuActionButton, MenuNavigationButton},
};

pub fn load_unit_buttons(age: Age, unit_type: UnitType) -> String {
    let mut res = String::from("unit/icon");

    match age {
        Age::StoneAge => res.push('1'),
        Age::Medival => res.push('2'),
        Age::Renaissance => res.push('3'),
        Age::Modern => res.push('4'),
        Age::Future => res.push('5'),
    }
    res.push('.');
    match unit_type {
        UnitType::Meele => res.push('1'),
        UnitType::Ranged => res.push('2'),
        UnitType::Tank => res.push('3'),
        UnitType::Super => {}
    }

    res.push_str(".png");

    return res;
}

pub fn load_turret_buttons(age: Age, turret_type: TurretType) -> String {
    let mut res = String::from("turret/iconweapon");

    match age {
        Age::StoneAge => res.push('1'),
        Age::Medival => res.push('2'),
        Age::Renaissance => res.push('3'),
        Age::Modern => res.push('4'),
        Age::Future => res.push('5'),
    }
    res.push('.');
    match turret_type {
        TurretType::Small => res.push('1'),
        TurretType::Medium => res.push('2'),
        TurretType::Large => res.push('3'),
    }

    res.push_str(".png");

    return res;
}

pub fn load_units(age: Age, unit_type: UnitType) -> String {
    let mut res = String::from("unit/");

    match age {
        Age::StoneAge => res.push('1'),
        Age::Medival => res.push('2'),
        Age::Renaissance => res.push('3'),
        Age::Modern => res.push('4'),
        Age::Future => res.push('5'),
    }
    res.push(' ');
    match unit_type {
        UnitType::Meele => res.push('1'),
        UnitType::Ranged => res.push('2'),
        UnitType::Tank => res.push('3'),
        UnitType::Super => {}
    }

    res.push_str(".png");

    return res;
}

pub fn load_base(age: Age) -> String {
    let mut res = String::from("base/");

    match age {
        Age::StoneAge => res.push('1'),
        Age::Medival => res.push('2'),
        Age::Renaissance => res.push('3'),
        Age::Modern => res.push('4'),
        Age::Future => res.push('5'),
    };

    res.push_str(".png");

    return res;
}

pub fn load_nav_icons(button: MenuNavigationButton) -> String {
    let mut res = String::from("menu/");

    res.push_str(match button {
        MenuNavigationButton::Unit => "unit",
        MenuNavigationButton::Turret | MenuNavigationButton::SelTurret => "turret",
        MenuNavigationButton::Back => "backarrow",
    });

    res.push_str(".png");

    return res;
}

pub fn load_action_icons(button: MenuActionButton) -> String {
    let mut res = String::from("menu/");

    res.push_str(match button {
        MenuActionButton::UpgradeBase => "turret",
        MenuActionButton::AdvanceAge => "advance",
    });

    res.push_str(".png");

    return res;
}

pub fn load_tower_part(index: usize, age: Age) -> String {
    let mut res = String::from("base/stage");

    res.push(match index {
        0 => '1',
        1 => '2',
        _ => '3',
    });
    res.push('.');
    res.push(match age {
        Age::StoneAge => '1',
        Age::Medival => '2',
        Age::Renaissance => '3',
        Age::Modern => '4',
        Age::Future => '5',
    });

    res.push_str(".png");

    debug!(res);
    return res;
}
