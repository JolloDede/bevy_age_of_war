use rand::{
    RngExt,
    distr::{Distribution, StandardUniform},
};

use crate::age_of_war::Age;

#[derive(Debug, PartialEq)]
pub struct GameUnit {
    pub level: Age,
    pub r#type: UnitType,
    pub hp: i32,
}

impl GameUnit {
    pub fn new(level: Age, typ: UnitType) -> Self {
        let hp = match typ {
            UnitType::Meele => 40,
            UnitType::Ranged => 20,
            UnitType::Tank => 80,
            UnitType::Super => 160,
        };
        Self {
            level: level,
            r#type: typ,
            hp: hp,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitType {
    Meele,
    Ranged,
    Tank,
    Super,
}

impl Distribution<UnitType> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> UnitType {
        match rng.random_range(0..4) {
            0 => UnitType::Meele,
            1 => UnitType::Ranged,
            2 => UnitType::Tank,
            _ => UnitType::Super,
        }
    }
}
