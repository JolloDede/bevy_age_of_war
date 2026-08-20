use std::{sync::Arc, time::Duration};

use bevy::prelude::*;

use crate::{
    consts::{QUEUE_MARGIN_LEFT, QUEUE_MARGIN_TOP, QUEUE_RECT_HEIGHT},
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
    mut timer_query: Query<(&mut QueueTimer, &Children)>,
    mut style_query: Query<&mut Node, With<ProgressbarFill>>,
) {
    for (mut bar, children) in timer_query.iter_mut() {
        if bar.unit.is_none() {
            return;
        }
        bar.timer.tick(time.delta());

        let percent = (1.0 - bar.timer.fraction()) * 100.0;
        for &child in children {
            if let Ok(mut node) = style_query.get_mut(child) {
                node.width = Val::Percent(percent);
            }
        }

        if bar.timer.just_finished() {
            commands.trigger(QueueTimerFinishedEvent);
        }
    }
}
