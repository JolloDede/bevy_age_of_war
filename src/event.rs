use bevy::prelude::*;

#[derive(Event)]
pub enum UnitSpawnEvent {
    Meele,
    Ranged,
    Tank,
    Super,
}
