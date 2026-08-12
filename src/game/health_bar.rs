use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{consts::*, game_unit::UnitType};

const HEALTH_BAR_SIZE: Vec2 = Vec2::new(200., 12.);
const HEALTH_BAR_MARGIN: f32 = 8.;
const HEALTH_BAR_HEALTH: Color = Color::srgb_u8(0, 255, 0);
const HEALTH_BAR_BACKGROUND: Color = Color::srgb_u8(255, 0, 0);

#[derive(Component, Deref)]
pub struct Health(pub i32);

impl From<UnitType> for Health {
    fn from(value: UnitType) -> Self {
        let hp = match value {
            UnitType::Meele => 90,
            UnitType::Ranged => 70,
            UnitType::Tank => 300,
            UnitType::Super => 160,
        };

        Self(hp)
    }
}

#[derive(Component, Deref)]
pub struct MaxHealth(pub i32);

#[derive(Component)]
pub struct HealthBarMarker;

pub fn health_bar_node(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    health: Health,
    is_base: bool,
) {
    let bar_y_offset = (BASE_SIZE.y / 2.) + HEALTH_BAR_MARGIN;
    let bar_x_offset = if is_base { 10. } else { 0. };
    let health_bar_size = if is_base {
        Vec2::new(BASE_SIZE.x / 2., 12.)
    } else {
        Vec2::new(
            UNIT_SIZE.x,
            UNIT_SIZE.x / HEALTH_BAR_SIZE.x * HEALTH_BAR_SIZE.y,
        )
    };

    parent
        .spawn((
            Sprite::from_color(HEALTH_BAR_BACKGROUND, health_bar_size),
            Transform::from_xyz(bar_x_offset, bar_y_offset, 1.0),
            MaxHealth(health.0),
            health,
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_color(HEALTH_BAR_HEALTH, health_bar_size),
                Transform::from_xyz(0., 0., 1.),
                HealthBarMarker,
            ));
        });
}
