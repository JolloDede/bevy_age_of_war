use std::sync::Arc;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    consts::{LEVEL_END, LEVEL_START},
    event::{BaseAdvanceAgeEvent, UnitSpawnEvent},
    game::{
        BASE_SIZE, GROUND_Y,
        base::{Base, Enemy},
    },
    game_unit::{GameUnit, UnitType},
};

#[derive(Component, Deref)]
pub struct UnitComp(pub Arc<GameUnit>);

const UNIT_SIZE: Vec2 = Vec2::new(20.0, 20.0);

pub fn unit_spawn_observer(spawn_event: On<UnitSpawnEvent>, mut commands: Commands) {
    debug!("Unit Spawn Event fired");

    const UNIT_COLOR: Color = Color::linear_rgb(1.0, 0.0, 1.0);

    let unit = spawn_event.0.clone();

    commands.spawn((
        Sprite::from_color(UNIT_COLOR, UNIT_SIZE),
        Transform::from_xyz(
            LEVEL_START + BASE_SIZE.x + (UNIT_SIZE.x * 0.5),
            GROUND_Y + (UNIT_SIZE.y * 0.5),
            1.0,
        ),
        Text2d::new(match unit.r#type {
            UnitType::Meele => "M",
            UnitType::Ranged => "R",
            UnitType::Tank => "T",
            UnitType::Super => "S",
        }),
        Intersects::default(),
        UnitComp(unit),
    ));
}

const UNIT_SPEED: f32 = 6.0;

pub fn unit_movement_system(mut unit_query: Query<(&mut Transform, &Intersects), With<UnitComp>>) {
    for (mut unit_trans, intersects) in unit_query.iter_mut() {
        if !intersects.0 {
            unit_trans.translation.x += UNIT_SPEED;
        }
    }
}

#[derive(Component, Default)]
pub struct Intersects(bool);

pub fn unit_collision_system(
    mut unit_query: Query<(&Transform, &mut Intersects), With<UnitComp>>,
    base_query: Single<(&Transform, &Base), With<Enemy>>,
) {
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

    let (base_trans, _) = base_query.into_inner();
    let base_aabb = Aabb2d::new(
        base_trans.translation.truncate(),
        (BASE_SIZE * base_trans.scale.truncate()) / 2.,
    );
    for (trans, mut intersects) in unit_query.iter_mut() {
        let unit_aabb = Aabb2d::new(
            trans.translation.truncate(),
            (UNIT_SIZE * trans.scale.truncate()) / 2.,
        );
        if unit_aabb.intersects(&base_aabb) {
            debug!("Unit intersected with base");
            intersects.0 = true;
        }
    }
}

pub fn advance_age_observer(advance_event: On<BaseAdvanceAgeEvent>) {
    debug!("Advance age event");
}
