use bevy::prelude::*;

use crate::game_unit::GameUnit;

#[derive(Event)]
pub enum UnitQueueEvent {
    Meele,
    Ranged,
    Tank,
    Super,
}

#[derive(Event, Deref)]
pub struct UnitSpawnEvent(pub GameUnit);

#[derive(Event)]
pub struct QueueTimerFinishedEvent;
