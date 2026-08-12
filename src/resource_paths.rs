use crate::{age_of_war::Age, game_turret::TurretType, game_unit::UnitType};

pub fn load_unit_buttons(age: Age, unit_type: UnitType) -> String {
    let mut res = String::from("unit/icon");

    match age {
        Age::StoneAge => res.push('1'),
        Age::Medival => res.push('2'),
        Age::Renaissance => res.push('3'),
        Age::Modern => res.push('4'),
        Age::Future => res.push('5'),
    }

    match unit_type {
        UnitType::Meele => res.push_str(".1"),
        UnitType::Ranged => res.push_str(".2"),
        UnitType::Tank => res.push_str(".3"),
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
