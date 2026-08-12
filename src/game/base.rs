use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    age_of_war::Age,
    consts::*,
    event::BaseAdvanceAgeEvent,
    game::{
        health_bar::{Health, MaxHealth, health_bar_node},
        unit::new_unit_comp,
    },
    game_unit::{GameUnit, UnitType},
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Base {
    pub age: Age,
}

impl Base {
    pub fn new() -> Self {
        Self { age: Age::StoneAge }
    }
}

#[derive(Component)]
pub struct EnemyBaseQueueTimer(Timer);

pub fn spawn_bases(mut commands: Commands) {
    let player_base_x = LEVEL_START + (BASE_SIZE.x * 0.5);
    let enemy_base_x = LEVEL_END - (BASE_SIZE.x * 0.5);
    let base_y = GROUND_Y + (BASE_SIZE.y * 0.5);

    commands
        .spawn((
            Sprite::from_color(BASE_COLOR, BASE_SIZE),
            Transform::from_xyz(player_base_x, base_y, 1.0),
            Base::new(),
        ))
        .with_children(|parent| {
            health_bar_node(parent, Health(UNIT_BASE_HEALTH), true);
        });

    commands
        .spawn((
            Sprite::from_color(BASE_COLOR, BASE_SIZE),
            Transform::from_xyz(enemy_base_x, base_y, 1.0),
            Base::new(),
            Enemy,
            EnemyBaseQueueTimer(Timer::from_seconds(4., TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            health_bar_node(parent, Health(UNIT_BASE_HEALTH), true);
        });
}

pub fn advance_age_observer(advance_event: On<BaseAdvanceAgeEvent>) {
    debug!("Advance age event");
}

pub fn enemy_base_spawn_unit(
    mut commands: Commands,
    time: Res<Time>,
    base_query: Single<(&mut EnemyBaseQueueTimer, &Base), With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    let (mut base_timer, base) = base_query.into_inner();

    base_timer.0.tick(time.delta());

    if base_timer.0.just_finished() {
        let typ = rand::random();
        let unit = GameUnit::new(base.age, typ);

        new_unit_comp(&mut commands, Arc::new(unit), true, asset_server);
    }
}
