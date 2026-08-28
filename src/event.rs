use std::sync::Arc;

use bevy::prelude::*;

use crate::game_unit::GameUnit;

#[derive(Event, Debug)]
pub struct UnitQueueEvent(pub Arc<GameUnit>);

#[derive(Event, Deref)]
pub struct UnitSpawnEvent(pub Arc<GameUnit>);

#[derive(Event)]
pub struct QueueTimerFinishedEvent;

#[derive(Event)]
pub struct BaseAdvanceAgeEvent;

#[derive(Event)]
pub struct UpgradeBaseEvent;

#[derive(Event)]
pub struct MarkTurretSpotsEvent;

#[derive(Event)]
pub struct UnMarkTurretSpotsEvent;
