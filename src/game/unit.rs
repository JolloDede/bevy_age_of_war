use std::sync::Arc;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    consts::*,
    event::{BaseAdvanceAgeEvent, UnitSpawnEvent},
    game::{
        BASE_SIZE, GROUND_Y,
        base::{Base, Enemy},
    },
    game_unit::{GameUnit, UnitType},
};

#[derive(Component, Deref)]
pub struct UnitComp(pub Arc<GameUnit>);

#[derive(Component, Deref)]
pub struct AttackCooldown(Timer);

pub fn unit_spawn_observer(spawn_event: On<UnitSpawnEvent>, mut commands: Commands) {
    debug!("Unit Spawn Event fired");

    let unit = spawn_event.0.clone();

    commands.spawn(new_unit_comp(unit));
}

const UNIT_SPEED: f32 = 6.0;

pub fn unit_movement_system(
    mut unit_query: Query<(&mut Transform, &Intersects, Entity), With<UnitComp>>,
    enemy_query: Query<&Enemy>,
) {
    for (mut unit_trans, intersects, entity) in unit_query.iter_mut() {
        if !intersects.0 {
            if enemy_query.get(entity).is_ok() {
                unit_trans.translation.x -= UNIT_SPEED;
            } else {
                unit_trans.translation.x += UNIT_SPEED;
            }
        }
    }
}

#[derive(Component, Default)]
pub struct Intersects(bool);

pub fn unit_collision_system(mut unit_query: Query<(&Transform, &mut Intersects), With<UnitComp>>) {
    let mut unit_trans_intersect = unit_query.iter_combinations_mut();
    while let Some([mut unit1, mut unit2]) = unit_trans_intersect.fetch_next() {
        let unit1_aabb = Aabb2d::new(
            unit1.0.translation.truncate(),
            (UNIT_SIZE * unit1.0.scale.truncate()) / 2.,
        );
        let unit2_aabb = Aabb2d::new(
            unit2.0.translation.truncate(),
            (UNIT_SIZE * unit2.0.scale.truncate()) / 2.,
        );
        if unit1_aabb.intersects(&unit2_aabb) {
            debug!("Unit intersected with oneanother");
            unit1.1.0 = true;
            unit2.1.0 = true;
        }
    }
}

pub fn base_collision_system(
    mut unit_query: Query<(&Transform, &mut Intersects), With<UnitComp>>,
    base_query: Query<&Transform, With<Base>>,
) {
    let mut base_aabbs = Vec::with_capacity(2);

    for base_trans in base_query.iter() {
        base_aabbs.push(Aabb2d::new(
            base_trans.translation.truncate(),
            (BASE_SIZE * base_trans.scale.truncate()) / 2.,
        ));
    }

    for (trans, mut intersects) in unit_query.iter_mut() {
        let unit_aabb = Aabb2d::new(
            trans.translation.truncate(),
            (UNIT_SIZE * trans.scale.truncate()) / 2.,
        );
        for base_aabb in &base_aabbs {
            if unit_aabb.intersects(base_aabb) {
                debug!("Unit intersected with base");
                intersects.0 = true;
            }
        }
    }
}

pub fn new_unit_comp(unit: Arc<GameUnit>) -> (Sprite, Transform, Text2d, Intersects, UnitComp) {
    let x_pos = LEVEL_START + BASE_SIZE.x + (UNIT_SIZE.x * 0.5) + 1.;

    (
        Sprite::from_color(UNIT_COLOR, UNIT_SIZE),
        Transform::from_xyz(x_pos, GROUND_Y + (UNIT_SIZE.y * 0.5), 1.0),
        Text2d::new(match unit.r#type {
            UnitType::Meele => "M",
            UnitType::Ranged => "R",
            UnitType::Tank => "T",
            UnitType::Super => "S",
        }),
        Intersects::default(),
        UnitComp(unit),
    )
}

pub fn new_enemy_unit_comp(
    unit: Arc<GameUnit>,
) -> (Sprite, Transform, Text2d, Intersects, UnitComp, Enemy) {
    let xpos = LEVEL_END - BASE_SIZE.x - (UNIT_SIZE.x * 0.5) - 1.;
    let (sprite, mut trans, text, intersect, unit) = new_unit_comp(unit);

    trans.translation.x = xpos;

    (sprite, trans, text, intersect, unit, Enemy)
}

pub fn draw_attack_ranges(units: Query<(&Transform, &UnitComp)>, mut gizmos: Gizmos) {
    for (transform, unit) in &units {
        let position = transform.translation.truncate();

        gizmos.circle_2d(
            position,
            unit.range * UNIT_SIZE.x,
            Color::srgb(1.0, 0.0, 0.0),
        );
    }
}

pub fn combat_system(time: Res<Time>, mut unit_query: Query<(&UnitComp, &mut AttackCooldown)>) {
    for (unit, mut cooldown) in unit_query.iter_mut() {
        cooldown.0.tick(time.delta());

        if !cooldown.is_finished() {
            continue;
        }
    }
}
