use bevy::{math::FloatPow, prelude::*};

use crate::{
    age_of_war::Age,
    consts::*,
    game::{base::Enemy, health_bar::Health, unit::UnitComp},
    game_unit::UnitType,
};

#[derive(Component, Deref)]
pub struct AttackCooldown(Timer);

#[derive(Component, Deref)]
pub struct AttackRange(f32);

#[derive(Component, Deref)]
pub struct AttackDamange(i32);

impl From<UnitType> for AttackRange {
    fn from(value: UnitType) -> Self {
        let mut range = match value {
            UnitType::Meele => 1.,
            UnitType::Ranged => 2.,
            UnitType::Tank => 1.,
            UnitType::Super => 3.,
        };

        range *= UNIT_SIZE.x;

        Self(range)
    }
}

impl AttackDamange {
    pub fn new(age: Age, unit_type: UnitType) -> Self {
        let mut damage = match unit_type {
            UnitType::Meele => 10,
            UnitType::Ranged => 20,
            UnitType::Tank => 40,
            UnitType::Super => 80,
        };
        damage *= match age {
            Age::StoneAge => 1,
            Age::Medival => 2,
            Age::Renaissance => 3,
            Age::Modern => 4,
            Age::Future => 5,
        };

        Self(damage)
    }
}

pub fn draw_attack_ranges(units: Query<(&Transform, &AttackRange)>, mut gizmos: Gizmos) {
    for (transform, range) in &units {
        let position = transform.translation.truncate();

        gizmos.circle_2d(position, range.0, Color::srgb(1.0, 0.0, 0.0));
    }
}

pub fn combat_system(
    time: Res<Time>,
    mut attacker_query: Query<(
        &Transform,
        &AttackRange,
        &AttackDamange,
        &mut AttackCooldown,
        Entity,
    )>,
    attacked_unit: Query<(Entity, &Transform), With<Health>>,
    enemy_query: Query<&Enemy>,
    mut health: Query<&mut Health>,
) {
    for (attacker_trans, attacker_range, attacker_damage, mut cooldown, entity) in
        attacker_query.iter_mut()
    {
        cooldown.0.tick(time.delta());

        if !cooldown.is_finished() {
            continue;
        }

        let attacker_pos = attacker_trans.translation.truncate();
        let range_squared = attacker_range.0.squared();
        let attacker_is_enemy = enemy_query.get(entity).is_ok();

        let mut nearest_enemy: Option<(Entity, f32)> = None;
        for (enemy_entity, enemy_trans) in attacked_unit.iter() {
            let is_enemy = enemy_query.get(enemy_entity).is_ok();
            if attacker_is_enemy == is_enemy {
                continue;
            }

            let enemy_pos = enemy_trans.translation.truncate();
            let distance_squared = attacker_pos.distance_squared(enemy_pos);
            if distance_squared <= range_squared {
                if let Some((_, nearest_distance)) = nearest_enemy {
                    if distance_squared < nearest_distance {
                        nearest_enemy = Some((enemy_entity, distance_squared));
                    }
                } else {
                    nearest_enemy = Some((enemy_entity, distance_squared));
                }
            }
        }

        if let Some((target_entity, _)) = nearest_enemy {
            if let Ok(mut health) = health.get_mut(target_entity) {
                health.0 -= attacker_damage.0;
                cooldown.0.reset();
                debug!("Attacked! remaining health: {}", health.0);
            }
        }
    }
}
