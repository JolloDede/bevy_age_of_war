use crate::age_of_war::Age;

#[derive(Debug, PartialEq)]
pub struct GameUnit {
    pub level: Age,
    pub r#type: UnitType,
    pub hp: i32,
    pub range: i32,
}

impl GameUnit {
    pub fn new(level: Age, typ: UnitType) -> Self {
        let (hp, range) = match typ {
            UnitType::Meele => (40, 1),
            UnitType::Ranged => (20, 2),
            UnitType::Tank => (80, 1),
            UnitType::Super => (160, 3),
        };
        Self {
            level: level,
            r#type: typ,
            hp: hp,
            range: range,
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
