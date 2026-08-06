use bevy::prelude::*;

use crate::age_of_war::Age;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Base {
    pub age: Age,
    pub hp: i32,
}

impl Base {
    pub fn new() -> Self {
        Self {
            age: Age::StoneAge,
            hp: 200,
        }
    }
}
