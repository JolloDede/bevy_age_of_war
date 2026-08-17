use bevy::{math::FloatPow, prelude::*};

use crate::{
    age_of_war::Age,
    consts::*,
    game::{HitBoxSize, base::Enemy, health_bar::Health, unit::UnitComp},
    game_unit::UnitType,
};

#[derive(Component, Deref)]
pub struct AttackCooldown(pub Timer);

#[derive(Component, Deref)]
pub struct AttackRange(f32);

impl From<UnitType> for AttackRange {
    fn from(value: UnitType) -> Self {
        let range = match value {
            UnitType::Meele => 60.,
            UnitType::Ranged => 100.,
            UnitType::Tank => 80.,
            UnitType::Super => 3.,
        };

        // range *= HitBoxSize::from(value).x * 1.5;

        Self(range)
    }
}

#[derive(Component, Deref)]
pub struct AttackDamange(i32);

impl AttackDamange {
    pub fn new(age: Age, unit_type: UnitType) -> Self {
        let mut damage = match unit_type {
            UnitType::Meele => 25,
            UnitType::Ranged => 9,
            UnitType::Tank => 40,
            UnitType::Super => 40,
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

pub fn draw_attack_ranges(
    mut gizmos: Gizmos,
    units: Query<(&Transform, &AttackRange, &Sprite)>,
    images: Res<Assets<Image>>,
) {
    for (trans, range, sprite) in &units {
        let Some(image) = images.get(&sprite.image) else {
            continue;
        };
        let position = Vec2::new(
            trans.translation.x,
            trans.translation.y + (image.height() as f32 / 4.),
        );

        gizmos.circle_2d(position, range.0, Color::srgb(1.0, 0.0, 0.0));
    }
}

pub fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut attacker_query: Query<(
        &Transform,
        &AttackRange,
        &AttackDamange,
        &mut AttackCooldown,
        Entity,
    )>,
    attacked_unit: Query<(Entity, &Transform), With<UnitComp>>,
    enemy_query: Query<&Enemy>,
    health_parent_query: Query<(&Children)>,
    mut health_query: Query<&mut Health>,
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
        for (attacked_entity, attacked_trans) in attacked_unit.iter() {
            let is_enemy = enemy_query.get(attacked_entity).is_ok();
            if attacker_is_enemy == is_enemy {
                continue;
            }

            let attacked_pos = attacked_trans.translation.truncate();
            let distance_squared = attacker_pos.distance_squared(attacked_pos);
            debug!(range_squared, distance_squared);
            if distance_squared <= range_squared {
                if let Some((_, nearest_distance)) = nearest_enemy {
                    if distance_squared < nearest_distance {
                        nearest_enemy = Some((attacked_entity, distance_squared));
                    }
                } else {
                    nearest_enemy = Some((attacked_entity, distance_squared));
                }
            }
        }

        if let Some((target_entity, _)) = nearest_enemy {
            let children = health_parent_query.get(target_entity).unwrap();
            for &child in children {
                if let Ok(mut health) = health_query.get_mut(child) {
                    health.0 -= attacker_damage.0;
                    if health.0 <= 0 {
                        info!("Killed a unit");
                        commands.entity(target_entity).despawn();
                    } else {
                        cooldown.0.reset();
                        debug!("Attacked! remaining health: {}", health.0);
                    }
                }
            }
        }
    }
}
