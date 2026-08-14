use std::sync::Arc;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    consts::*,
    event::{BaseAdvanceAgeEvent, UnitSpawnEvent},
    game::{
        BASE_SIZE, GROUND_Y, HitBoxSize,
        base::{Base, Enemy},
        combat::{AttackCooldown, AttackDamange, AttackRange},
        health_bar::{Health, health_bar_node},
    },
    game_unit::{GameUnit, UnitType},
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

pub fn unit_collision_system(
    mut unit_query: Query<(&Transform, &mut Intersects, &HitBoxSize), With<UnitComp>>,
) {
    let mut unit_trans_intersect = unit_query.iter_combinations_mut();
    while let Some([mut unit1, mut unit2]) = unit_trans_intersect.fetch_next() {
        let unit1_aabb = Aabb2d::new(
            unit1.0.translation.truncate(),
            (unit1.2.0 * unit1.0.scale.truncate()) / 2.,
        );
        let unit2_aabb = Aabb2d::new(
            unit2.0.translation.truncate(),
            (unit1.2.0 * unit2.0.scale.truncate()) / 2.,
        );
        if unit1_aabb.intersects(&unit2_aabb) {
            debug!("Unit intersected with oneanother");
            unit1.1.0 = true;
            unit2.1.0 = true;
        }
    }
}

pub fn base_collision_system(
    mut unit_query: Query<(&Transform, &mut Intersects, &HitBoxSize), With<UnitComp>>,
    base_query: Query<&Transform, With<Base>>,
) {
    let mut base_aabbs = Vec::with_capacity(2);

    for base_trans in base_query.iter() {
        base_aabbs.push(Aabb2d::new(
            base_trans.translation.truncate(),
            (BASE_SIZE * base_trans.scale.truncate()) / 2.,
        ));
    }

    for (trans, mut intersects, hitbox) in unit_query.iter_mut() {
        let unit_aabb = Aabb2d::new(
            trans.translation.truncate(),
            (hitbox.0 * trans.scale.truncate()) / 2.,
        );
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
    let x_pos = if is_enemy {
        LEVEL_END - BASE_SIZE.x - (hitbox.x * 0.5) - 1.
    } else {
        LEVEL_START + BASE_SIZE.x + (hitbox.x * 0.5) + 1.
    };
    let mut sprite =
        Sprite::from_image(asset_server.load(resource_paths::load_units(unit.level, unit.r#type)));
    if is_enemy {
        sprite.flip_x = true;
    }

    let mut unit_bundle = commands.spawn((
        sprite,
        Transform::from_xyz(x_pos, GROUND_Y, 1.0).with_scale(Vec3::splat(1.5)),
        Intersects::default(),
        AttackRange::from(unit.r#type),
        AttackDamange::new(unit.level, unit.r#type),
        AttackCooldown(Timer::from_seconds(1., TimerMode::Once)),
        UnitComp(unit.clone()),
        hitbox,
    ));
    if is_enemy {
        unit_bundle.insert(Enemy);
    }

    unit_bundle.with_children(|parent| {
        health_bar_node(parent, Health::from(unit.r#type), false);
    });
}
