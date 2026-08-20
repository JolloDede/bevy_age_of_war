use std::{sync::Arc, time::Duration};

use bevy::prelude::*;

use crate::{
    event::QueueTimerFinishedEvent,
    game_unit::{GameUnit, UnitType},
};

#[derive(Component)]
pub struct QueueTimer {
    pub timer: Timer,
    pub unit: Option<Arc<GameUnit>>,
}

impl QueueTimer {
    pub fn set_unit(&mut self, unit: Arc<GameUnit>) {
        self.timer.set_duration(match unit.r#type {
            UnitType::Meele => Duration::from_secs_f32(0.8),
            UnitType::Ranged => Duration::from_secs(1),
            UnitType::Tank => Duration::from_secs(2),
            UnitType::Super => Duration::from_secs(8),
        });
        self.timer.reset();
        self.unit = Some(unit);
    }
}

#[derive(Component)]
pub struct ProgressbarFill;

pub fn progressbar_system(
    mut commands: Commands,
    time: Res<Time>,
    timer_query: Single<&mut QueueTimer>,
    style_query: Single<&mut Node, With<ProgressbarFill>>,
) {
    let mut bar = timer_query.into_inner();

    if bar.unit.is_none() {
        return;
    }

    bar.timer.tick(time.delta());

    let percent = bar.timer.fraction() * 100.0;
    let mut node = style_query.into_inner();
    node.width = Val::Percent(percent);

    if bar.timer.just_finished() {
        node.width = Val::ZERO;
        commands.trigger(QueueTimerFinishedEvent);
    }
}
