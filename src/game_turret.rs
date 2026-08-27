use bevy::prelude::*;

use strum::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum TurretType {
    Small,
    Medium,
    Large,
}

#[derive(Component)]
pub struct BaseTower(pub usize);
