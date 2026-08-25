use crate::{age_of_war::Age, game_turret::TurretType, game_unit::UnitType};

pub fn unit_name(unit: UnitType, age: Age) -> String {
    String::from(match (unit, age) {
        (UnitType::Meele, Age::StoneAge) => "Club man",
        (UnitType::Ranged, Age::StoneAge) => "Sling man",
        (UnitType::Tank, Age::StoneAge) => "Dino rider",
        (UnitType::Super, Age::StoneAge) => "",

        (UnitType::Meele, Age::Medival) => "Sword man",
        (UnitType::Ranged, Age::Medival) => "Archer",
        (UnitType::Tank, Age::Medival) => "Knight",
        (UnitType::Super, Age::Medival) => "",

        (UnitType::Meele, Age::Renaissance) => "Dueler",
        (UnitType::Ranged, Age::Renaissance) => "Moutsquettere",
        (UnitType::Tank, Age::Renaissance) => "Canoneer",
        (UnitType::Super, Age::Renaissance) => "",

        (UnitType::Meele, Age::Modern) => "Meele Infantry",
        (UnitType::Ranged, Age::Modern) => "Infantry",
        (UnitType::Tank, Age::Modern) => "Tank",
        (UnitType::Super, Age::Modern) => "",

        (UnitType::Meele, Age::Future) => "God's Blade",
        (UnitType::Ranged, Age::Future) => "Blaster",
        (UnitType::Tank, Age::Future) => "War Machine",
        (UnitType::Super, Age::Future) => "",
    })
}

pub fn turret_name(turret: TurretType, age: Age) -> String {
    String::from(match (turret, age) {
        (TurretType::Small, Age::StoneAge) => "Rock slingshot",
        (TurretType::Medium, Age::StoneAge) => "Egg Automatik",
        (TurretType::Large, Age::StoneAge) => "Primitive Catapult",

        (TurretType::Small, Age::Medival) => "Catapult",
        (TurretType::Medium, Age::Medival) => "Fire Catapult",
        (TurretType::Large, Age::Medival) => "Oil",

        (TurretType::Small, Age::Renaissance) => "Small Canon",
        (TurretType::Medium, Age::Renaissance) => "Large Canon",
        (TurretType::Large, Age::Renaissance) => "Explosiv Canon",

        (TurretType::Small, Age::Modern) => "Single Turret",
        (TurretType::Medium, Age::Modern) => "Rocket Turret",
        (TurretType::Large, Age::Modern) => "Double Turret",

        (TurretType::Small, Age::Future) => "Titanium Shooter",
        (TurretType::Medium, Age::Future) => "Lazer Canon",
        (TurretType::Large, Age::Future) => "Ion Ray",
    })
}
