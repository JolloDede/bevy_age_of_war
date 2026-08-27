use std::sync::Arc;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    Base, Enemy,
    consts::*,
    event::UnitSpawnEvent,
    game::{
        GROUND_Y, HitBoxSize,
        combat::{AttackCooldown, AttackDamange, AttackRange},
        health_bar::{Health, health_bar_node},
    },
    game_unit::GameUnit,
    resource_paths,
};

#[derive(Component, Deref)]
pub struct UnitComp(pub Arc<GameUnit>);

pub fn unit_spawn_observer(
    spawn_event: On<UnitSpawnEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    debug!("Unit Spawn Event fired");

    let unit = spawn_event.0.clone();

    new_unit_comp(&mut commands, unit, false, asset_server);
}

const UNIT_SPEED: f32 = 3.0;

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

pub fn unit_collision_system(
    mut unit_query: Query<(&Transform, &mut Intersects, &HitBoxSize, Entity), With<UnitComp>>,
    enemy_query: Query<&Enemy>,
) {
    let mut unit_trans_intersect = unit_query.iter_combinations_mut();
    while let Some([mut unit1, mut unit2]) = unit_trans_intersect.fetch_next() {
        let unit1_aabb = Aabb2d::new(unit1.0.translation.truncate(), unit1.2.0 * 0.5);
        let unit2_aabb = Aabb2d::new(unit2.0.translation.truncate(), unit2.2.0 * 0.5);
        if unit1_aabb.intersects(&unit2_aabb) {
            let is_unit1_enemy = enemy_query.get(unit1.3).is_ok();
            let is_unit2_enemy = enemy_query.get(unit2.3).is_ok();
            // Check if enemy
            if is_unit1_enemy == is_unit2_enemy {
                // check if its the first if they match enemy
                match is_unit1_enemy {
                    // only stop the later
                    true => {
                        if unit1_aabb.min.x > unit2_aabb.min.x {
                            unit1.1.0 = true;
                        } else {
                            unit2.1.0 = true;
                        }
                    }
                    false => {
                        if unit1_aabb.max.x > unit2_aabb.max.x {
                            unit2.1.0 = true;
                        } else {
                            unit1.1.0 = true;
                        }
                    }
                }
                continue;
            }
            unit1.1.0 = true;
            unit2.1.0 = true;
        }
    }
}

pub fn base_collision_system(
    mut unit_query: Query<(&Transform, &mut Intersects, &HitBoxSize), With<UnitComp>>,
    base_query: Query<(&Transform, &HitBoxSize), With<Base>>,
) {
    let mut base_aabbs = Vec::with_capacity(2);

    for (base_trans, base_hitbox) in base_query.iter() {
        base_aabbs.push(Aabb2d::new(
            base_trans.translation.truncate(),
            base_hitbox.0 * 0.5,
        ));
    }

    for (trans, mut intersects, hitbox) in unit_query.iter_mut() {
        let unit_aabb = Aabb2d::new(trans.translation.truncate(), hitbox.0 * 0.5);
        for base_aabb in &base_aabbs {
            if unit_aabb.intersects(base_aabb) {
                debug!("Unit intersected with base");
                intersects.0 = true;
            }
        }
    }
}

pub fn clear_unit_collision(mut unit_query: Query<&mut Intersects, With<UnitComp>>) {
    for mut intersects in unit_query.iter_mut() {
        intersects.0 = false;
    }
}

pub fn new_unit_comp(
    commands: &mut Commands,
    unit: Arc<GameUnit>,
    is_enemy: bool,
    asset_server: Res<AssetServer>,
) {
    let hitbox = HitBoxSize::from(unit.r#type);
    let unit_size = hitbox.0.clone();
    let base_size = HitBoxSize::new_base();

    let x_pos = if is_enemy {
        LEVEL_END - base_size.x
    } else {
        LEVEL_START + (base_size.x * 0.8) + hitbox.x / 2.
    };
    let mut sprite =
        Sprite::from_image(asset_server.load(resource_paths::load_units(unit.age, unit.r#type)));
    if is_enemy {
        sprite.flip_x = true;
    }

    let mut unit_bundle = commands.spawn((
        sprite,
        Transform::from_xyz(x_pos, GROUND_Y, 1.0).with_scale(Vec3::splat(1.5)),
        Intersects::default(),
        AttackRange::from(unit.r#type),
        AttackDamange::new(unit.age, unit.r#type),
        AttackCooldown(Timer::from_seconds(1., TimerMode::Once)),
        UnitComp(unit.clone()),
        hitbox,
    ));
    if is_enemy {
        unit_bundle.insert(Enemy);
    }

    unit_bundle.with_children(|parent| {
        health_bar_node(parent, Health::from(unit.r#type), Some(unit_size));
    });
}
