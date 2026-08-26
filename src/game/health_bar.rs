use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*, sprite::Anchor};

use crate::{consts::*, game::base::Base, game_unit::UnitType};

const HEALTH_BAR_SIZE: Vec2 = Vec2::new(200., 4.);
const HEALTH_BAR_MARGIN: f32 = 12.;
const HEALTH_BAR_HEALTH: Color = Color::srgb_u8(0, 255, 0);
const HEALTH_BAR_BACKGROUND: Color = Color::srgb_u8(255, 0, 0);
const BASE_HEALTH_BAR_HEALTH: Color = Color::srgb_u8(255, 0, 0);
const HEALTH_BAR_BORDER: Color = Color::BLACK;

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
    unit_size: Option<Vec2>,
) {
    let (health_bar_size, health_bar_color, health_bar_bg_color, bar_offset) = match unit_size {
        None => (
            Vec2::new(BASE_SIZE.x / 2., BASE_HEALTH_BAR_HEIGHT),
            BASE_HEALTH_BAR_HEALTH,
            Color::srgba_u8(0, 0, 0, 11),
            Vec2::new(0., BASE_SIZE.y + HEALTH_BAR_MARGIN),
        ),
        Some(unit_size) => (
            Vec2::new(unit_size.x / 2., HEALTH_BAR_SIZE.y),
            HEALTH_BAR_HEALTH,
            HEALTH_BAR_BACKGROUND,
            Vec2::new(0., (unit_size.y / 2.) + HEALTH_BAR_MARGIN),
        ),
    };

    let health_bar_bundle = (
        Sprite::from_color(health_bar_bg_color, health_bar_size),
        Transform::from_xyz(bar_offset.x, bar_offset.y, 1.0),
        MaxHealth(health.0),
        health,
    );

    let mut hb_bg = if unit_size.is_none() {
        parent.spawn((health_bar_bundle,))
    } else {
        parent.spawn(health_bar_bundle)
    };

    hb_bg.with_children(|parent| {
        if unit_size.is_none() {
            let border_thickness = 1.;
            // Top Border
            parent.spawn((
                Sprite::from_color(
                    HEALTH_BAR_BORDER,
                    Vec2::new(health_bar_size.x + border_thickness * 2.0, border_thickness),
                ),
                Transform::from_xyz(0.0, health_bar_size.y / 2.0 + border_thickness / 2.0, 0.1),
            ));
            // Bottom Border
            parent.spawn((
                Sprite::from_color(
                    HEALTH_BAR_BORDER,
                    Vec2::new(health_bar_size.x + border_thickness * 2.0, border_thickness),
                ),
                Transform::from_xyz(0.0, -health_bar_size.y / 2.0 - border_thickness / 2.0, 0.1),
            ));
            // Left Border
            parent.spawn((
                Sprite::from_color(
                    HEALTH_BAR_BORDER,
                    Vec2::new(border_thickness, health_bar_size.y),
                ),
                Transform::from_xyz(-health_bar_size.x / 2.0 - border_thickness / 2.0, 0.0, 0.1),
            ));
            // Right Border
            parent.spawn((
                Sprite::from_color(
                    HEALTH_BAR_BORDER,
                    Vec2::new(border_thickness, health_bar_size.y),
                ),
                Transform::from_xyz(health_bar_size.x / 2.0 + border_thickness / 2.0, 0.0, 0.1),
            ));
        }

        parent.spawn((
            Sprite::from_color(health_bar_color, health_bar_size),
            Transform::from_xyz(-(health_bar_size.x / 2.), -(health_bar_size.y / 2.), 1.),
            Anchor::BOTTOM_LEFT,
            HealthBarMarker,
        ));
    });
}

pub fn health_system(
    parent_query: Query<(&Health, &MaxHealth, &Children, &ChildOf)>,
    mut health_query: Query<&mut Transform, With<HealthBarMarker>>,
    base_query: Query<&Base>,
) {
    for (health, max_health, children, childof) in parent_query.iter() {
        let health_frac = health.0 as f32 / max_health.0 as f32;
        for child in children {
            if let Ok(mut item) = health_query.get_mut(*child) {
                match base_query.get(childof.parent()) {
                    Ok(_) => {
                        item.scale.y = health_frac;
                    }
                    Err(_) => item.scale.x = health_frac,
                }
            }
        }
    }
}
