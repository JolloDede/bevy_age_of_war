use std::ops::Div;

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::{
    age_of_war::Age,
    game::{
        HitBoxSize,
        base::{Base, Enemy},
        health_bar::Health,
        unit::UnitComp,
    },
    game_unit::UnitType,
    player::{Experience, GameFinishedEvent, Money},
};

#[derive(Component, Deref)]
pub struct AttackCooldown(pub Timer);

#[derive(Component, Deref)]
pub struct AttackRange(f32);

impl From<UnitType> for AttackRange {
    fn from(value: UnitType) -> Self {
        let range = match value {
            UnitType::Meele => 40.,
            UnitType::Ranged => 120.,
            UnitType::Tank => 80.,
            UnitType::Super => 3.,
        };

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
    attacked_unit: Query<(Entity, &Transform, &HitBoxSize, &Sprite, &UnitComp)>,
    attacked_base: Query<(Entity, &Transform, &HitBoxSize, &Sprite), With<Base>>,
    enemy_query: Query<&Enemy>,
    health_parent_query: Query<&Children>,
    mut health_query: Query<&mut Health>,
    images: Res<Assets<Image>>,
    mut money: ResMut<Money>,
    mut experience: ResMut<Experience>,
) {
    for (attacker_trans, attacker_range, attacker_damage, mut cooldown, entity) in
        attacker_query.iter_mut()
    {
        cooldown.0.tick(time.delta());

        if !cooldown.is_finished() {
            continue;
        }

        let attacker_radius =
            BoundingCircle::new(attacker_trans.translation.truncate(), attacker_range.0);
        let attacker_is_enemy = enemy_query.get(entity).is_ok();

        let mut nearest_enemy: Option<(Entity, f32, Option<(UnitType, Age)>)> = None;
        for (attacked_entity, attacked_trans, hitbox, sprite, unit) in attacked_unit.iter() {
            let is_enemy = enemy_query.get(attacked_entity).is_ok();
            if attacker_is_enemy == is_enemy {
                continue;
            }

            let Some(image) = images.get(&sprite.image) else {
                continue;
            };

            let pos = Vec2::new(
                attacked_trans.translation.x,
                attacked_trans.translation.y + (image.height() as f32 / 4.),
            );
            let attacked_hitbox = Aabb2d::new(pos, hitbox.div(2.));
            if attacker_radius.intersects(&attacked_hitbox) {
                let distance = if is_enemy {
                    attacker_radius.center.x - hitbox.x.div(2.)
                } else {
                    attacker_radius.center.x + hitbox.x.div(2.)
                };
                if let Some((_, nearest_distance, _)) = nearest_enemy {
                    if distance < nearest_distance {
                        nearest_enemy =
                            Some((attacked_entity, distance, Some((unit.r#type, unit.age))));
                    }
                } else {
                    nearest_enemy =
                        Some((attacked_entity, distance, Some((unit.r#type, unit.age))));
                }
            }
        }

        if nearest_enemy.is_none() {
            for (base_entity, base_trans, hitbox, sprite) in attacked_base.iter() {
                let is_enemy = enemy_query.get(base_entity).is_ok();
                if attacker_is_enemy == is_enemy {
                    continue;
                }

                let Some(image) = images.get(&sprite.image) else {
                    continue;
                };

                let pos = Vec2::new(
                    base_trans.translation.x,
                    base_trans.translation.y + (image.height() as f32 / 4.),
                );
                let attacked_hitbox = Aabb2d::new(pos, hitbox.div(2.));
                if attacker_radius.intersects(&attacked_hitbox) {
                    debug!("Attack enemy base");
                    let distance = if is_enemy {
                        attacker_radius.center.x - hitbox.x.div(2.)
                    } else {
                        attacker_radius.center.x + hitbox.x.div(2.)
                    };
                    nearest_enemy = Some((base_entity, distance, None));
                }
            }
        }

        if let Some((target_entity, _, is_unit)) = nearest_enemy {
            let children = health_parent_query.get(target_entity).unwrap();
            for &child in children {
                if let Ok(mut health) = health_query.get_mut(child) {
                    health.0 -= attacker_damage.0;
                    if health.0 <= 0 {
                        if let Some((u_type, age)) = is_unit {
                            info!("Killed a unit");
                            if !attacker_is_enemy {
                                money.killed_unit(u_type, age);
                                experience.killed_unit(u_type, age);
                            }
                            commands.entity(target_entity).despawn();
                        } else {
                            info!("Killed a base");
                            commands.trigger(GameFinishedEvent(target_entity));
                        }
                    } else {
                        cooldown.0.reset();
                        debug!("Attacked! remaining health: {}", health.0);
                    }
                }
            }
        }
    }
}
