use bevy::prelude::*;

use crate::game_unit::GameUnit;

#[derive(Event, Debug)]
pub struct UnitQueueEvent(pub GameUnit);

#[derive(Event, Deref)]
pub struct UnitSpawnEvent(pub GameUnit);

#[derive(Event)]
pub struct QueueTimerFinishedEvent;
