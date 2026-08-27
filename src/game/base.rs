use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    age_of_war::Age,
    consts::*,
    event::{BaseAdvanceAgeEvent, UpgradeBaseEvent},
    game::{
        GameMarker, HitBoxSize,
        health_bar::{Health, health_bar_node},
        unit::new_unit_comp,
    },
    game_turret::BaseTower,
    game_unit::GameUnit,
    hud::BaseAge,
    player::Money,
    resource_paths::{self, load_tower_part},
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

#[derive(Component)]
pub struct BaseHealthTextMarker;

pub fn spawn_bases(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_base_x = LEVEL_START + (BASE_SIZE.x * 0.8) + BASE_MARGIN;
    let enemy_base_x = LEVEL_END - (BASE_SIZE.x * 0.5) - BASE_MARGIN;
    let base_y = GROUND_Y + (BASE_SIZE.y * 0.5);

    commands
        .spawn((
            Sprite::from_image(asset_server.load(resource_paths::load_base(Age::StoneAge))),
            Transform::from_xyz(player_base_x, base_y, 1.0),
            Base::new(),
            HitBoxSize::new_base(),
            GameMarker,
        ))
        .with_children(|parent| {
            let y_offset = -24.;
            let sprite_size = 35.;
            parent.spawn((
                Sprite::from(asset_server.load("base/turret_place.png")),
                Transform::from_xyz(BASE_SIZE.x / 3. + 4., (BASE_SIZE.y / 4.) + y_offset, 3.),
                children![Sprite::from_color(
                    Color::srgba_u8(255, 255, 255, 100),
                    Vec2::new(sprite_size, sprite_size)
                ),],
            ));
            for (index, asset) in [
                "base/stage1.1.png",
                "base/stage2.1.png",
                "base/stage3.1.png",
            ]
            .iter()
            .enumerate()
            {
                // parent.spawn((
                //     Sprite::from(asset_server.load(*asset)),
                //     Transform::from_xyz(
                //         BASE_SIZE.x / 3. + 4.,
                //         (BASE_SIZE.y / 2.) + (index as f32 * BASE_EXPAND_SIZE.y) + y_offset,
                //         2.,
                //     ),
                //     BaseTower(index),
                // ));
                parent.spawn((
                    Sprite::from(asset_server.load("base/turret_place.png")),
                    Transform::from_xyz(
                        BASE_SIZE.x / 3. + 4.,
                        (BASE_SIZE.y / 2.) + (index as f32 * BASE_EXPAND_SIZE.y) + y_offset,
                        3.,
                    ),
                ));
            }

            parent.spawn((
                Text2d::new(BASE_START_HEALTH.to_string()),
                TextColor::from(Color::srgb_u8(255, 0, 0)),
                Transform::from_xyz(
                    BASE_SIZE.x / 4.,
                    BASE_SIZE.y + (BASE_HEALTH_BAR_HEIGHT / 2.),
                    0.,
                ),
                BaseHealthTextMarker,
            ));
            health_bar_node(parent, Health(BASE_START_HEALTH), None);
        });

    commands
        .spawn((
            Sprite {
                image: asset_server.load(resource_paths::load_base(Age::StoneAge)),
                flip_x: true,
                ..Default::default()
            },
            Transform::from_xyz(enemy_base_x, base_y, 1.0),
            Base::new(),
            HitBoxSize::new_base(),
            Enemy,
            EnemyBaseQueueTimer(Timer::from_seconds(4., TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new(BASE_START_HEALTH.to_string()),
                TextColor::from(Color::srgb_u8(255, 0, 0)),
                Transform::from_xyz(
                    -(BASE_SIZE.x / 2.),
                    BASE_SIZE.y + (BASE_HEALTH_BAR_HEIGHT / 2.),
                    0.,
                ),
                BaseHealthTextMarker,
            ));
            health_bar_node(parent, Health(BASE_START_HEALTH), None);
        });
}

pub fn base_health_text(
    mut text_query: Query<(&mut Text2d, &ChildOf), With<BaseHealthTextMarker>>,
    base_query: Query<&Children, With<Base>>,
    health_query: Query<&Health>,
) {
    for (mut text, childof) in text_query.iter_mut() {
        let children = base_query.get(childof.parent()).unwrap();
        for child in children {
            if let Ok(health) = health_query.get(*child) {
                text.0 = health.0.to_string();
            }
        }
    }
}

pub fn advance_age_observer(
    _advance_event: On<BaseAdvanceAgeEvent>,
    asset_server: Res<AssetServer>,
    base_age: Res<BaseAge>,
    mut sprite_query: ParamSet<(
        Single<&mut Sprite, (With<Base>, Without<Enemy>)>,
        Query<(&mut Sprite, &BaseTower, &mut Transform)>,
    )>,
) {
    let mut sprite = sprite_query.p0().into_inner();

    sprite.image = asset_server.load(resource_paths::load_base(base_age.0));

    for (mut sprite, base_tower, mut trans) in sprite_query.p1().iter_mut() {
        if base_tower.0 == 0 {
            if base_age.0 == Age::Modern {
                trans.translation.x -= 11.;
            }
            if base_age.0 == Age::Future {
                trans.translation.x += 13.;
            }
        }
        sprite.image = asset_server.load(resource_paths::load_tower_part(base_tower.0, base_age.0));
    }
}

pub fn upgrade_base_observer(
    _upgrade_event: On<UpgradeBaseEvent>,
    mut commands: Commands,
    base_query: Single<Entity, (With<Base>, Without<Enemy>)>,
    turret_query: Query<Entity, With<BaseTower>>,
    asset_server: Res<AssetServer>,
    base_age: Res<BaseAge>,
    mut money: ResMut<Money>,
) {
    let y_offset = -24.;

    let tower_stage = turret_query.count();
    if tower_stage == BASE_MAX_TOWER_COUNT {
        debug!("Cant build anymore towers");
        return;
    }
    let amount = money.tower_upgrade(tower_stage);

    if money.subtract_money(amount) {
        let base_entity = base_query.into_inner();
        let tower_part = (
            Sprite::from(asset_server.load(load_tower_part(tower_stage, base_age.0))),
            Transform::from_xyz(
                BASE_SIZE.x / 3. + 4.,
                (BASE_SIZE.y / 2.) + (tower_stage as f32 * BASE_EXPAND_SIZE.y) + y_offset,
                2.,
            ),
            BaseTower(tower_stage),
        );
        commands.entity(base_entity).with_child(tower_part);
    }
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
