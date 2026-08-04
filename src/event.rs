use bevy::prelude::*;

#[derive(Event)]
pub enum UnitQueueEvent {
    Meele,
    Ranged,
    Tank,
    Super,
}
