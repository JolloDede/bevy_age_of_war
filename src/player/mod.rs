use bevy::prelude::*;

use crate::{age_of_war::Age, game_unit::UnitType};

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
