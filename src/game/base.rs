use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Base {
    pub level: BaseLevel,
    pub hp: i32,
}

impl Base {
    pub fn new() -> Self {
        Self {
            level: BaseLevel::StoneAge,
            hp: 200,
        }
    }
}

pub enum BaseLevel {
    StoneAge,
    Medival,
    Renaissance,
    Current,
    Future,
}
