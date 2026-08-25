use bevy::prelude::*;

use crate::{age_of_war::Age, game_turret::TurretType, game_unit::UnitType};

#[derive(Event)]
pub struct GameFinishedEvent(pub Entity);

#[derive(Resource)]
pub struct Money(pub u32);

#[derive(Resource)]
pub struct Experience(pub u32);

impl Money {
    pub fn killed_unit(&mut self, unit_type: UnitType, age: Age) {
        let mut amount = match unit_type {
            UnitType::Meele => 20,
            UnitType::Ranged => 30,
            UnitType::Tank => 40,
            UnitType::Super => 100,
        };

        amount *= match age {
            Age::StoneAge => 1,
            Age::Medival => 2,
            Age::Renaissance => 3,
            Age::Modern => 4,
            Age::Future => 5,
        };

        self.0 += amount;
    }

    pub fn unit_price(&self, unit: UnitType, age: Age) -> u32 {
        match (unit, age) {
            (UnitType::Meele, Age::StoneAge) => 15,
            (UnitType::Ranged, Age::StoneAge) => 25,
            (UnitType::Tank, Age::StoneAge) => 100,
            (UnitType::Super, Age::StoneAge) => todo!(),

            (UnitType::Meele, Age::Medival) => 50,
            (UnitType::Ranged, Age::Medival) => 75,
            (UnitType::Tank, Age::Medival) => 500,
            (UnitType::Super, Age::Medival) => todo!(),

            (UnitType::Meele, Age::Renaissance) => 200,
            (UnitType::Ranged, Age::Renaissance) => 400,
            (UnitType::Tank, Age::Renaissance) => 1000,
            (UnitType::Super, Age::Renaissance) => todo!(),

            (UnitType::Meele, Age::Modern) => 1500,
            (UnitType::Ranged, Age::Modern) => 2000,
            (UnitType::Tank, Age::Modern) => 7000,
            (UnitType::Super, Age::Modern) => todo!(),

            (UnitType::Meele, Age::Future) => 5000,
            (UnitType::Ranged, Age::Future) => 6000,
            (UnitType::Tank, Age::Future) => 20000,
            (UnitType::Super, Age::Future) => todo!(),
        }
    }

    pub fn subtract_money(&mut self, amount: u32) -> bool {
        let mut subtracted = false;
        if amount <= self.0 {
            self.0 -= amount;
            subtracted = true;
        }

        return subtracted;
    }

    pub fn tower_price(&self, turret: TurretType, age: Age) -> u32 {
        match (turret, age) {
            (TurretType::Small, Age::StoneAge) => 100,
            (TurretType::Medium, Age::StoneAge) => 200,
            (TurretType::Large, Age::StoneAge) => 500,

            (TurretType::Small, Age::Medival) => 500,
            (TurretType::Medium, Age::Medival) => 750,
            (TurretType::Large, Age::Medival) => 1000,

            (TurretType::Small, Age::Renaissance) => 1500,
            (TurretType::Medium, Age::Renaissance) => 3000,
            (TurretType::Large, Age::Renaissance) => 6000,

            (TurretType::Small, Age::Modern) => 7000,
            (TurretType::Medium, Age::Modern) => 9000,
            (TurretType::Large, Age::Modern) => 14000,

            (TurretType::Small, Age::Future) => 24_000,
            (TurretType::Medium, Age::Future) => 40_000,
            (TurretType::Large, Age::Future) => 100_000,
        }
    }
}

impl Experience {
    pub fn killed_unit(&mut self, unit_type: UnitType, age: Age) {
        let mut amount = match unit_type {
            UnitType::Meele => 40,
            UnitType::Ranged => 60,
            UnitType::Tank => 80,
            UnitType::Super => 100,
        };

        amount *= match age {
            Age::StoneAge => 1,
            Age::Medival => 2,
            Age::Renaissance => 3,
            Age::Modern => 4,
            Age::Future => 5,
        };

        self.0 += amount;
    }
}
