use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{consts::*, event::UnitSpawnEvent, game_unit::GameUnit};

mod base;
use base::*;
mod unit;
use unit::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, move_camera);

        app.add_systems(Startup, spawn_world);

        app.add_systems(Startup, spawn_bases);
        app.add_systems(Update, enemy_base_spawn_unit);
        app.add_observer(advance_age_observer);

        app.add_systems(Update, unit_movement_system);
        app.add_systems(Update, unit_collision_system);
        // app.add_systems(Update, unit_range_system);
        app.add_systems(Update, draw_attack_ranges);
        app.add_systems(Update, base_collision_system);
        app.add_observer(unit_spawn_observer);
    }
}

#[derive(Component)]
struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(LEVEL_START, 0.0, 1.0),
        RenderLayers::layer(GAME_LAYER),
        GameCamera,
    ));
}

fn spawn_world(mut commands: Commands) {
    // horizon
    commands
        .spawn(Sprite::from_color(
            Color::srgb(0.0, 0.0, 1.0),
            Vec2::new(LEVEL_WIDTH, 200.0),
        ))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    // vegetation
    commands
        .spawn(Sprite::from_color(
            Color::srgb(0.0, 1.0, 0.0),
            Vec2::new(LEVEL_WIDTH, 100.0),
        ))
        .insert(Transform::from_xyz(0.0, -50.0, 0.0));
    // ground
    commands
        .spawn(Sprite::from_color(
            Color::srgb(0.5, 0.1, 0.1),
            Vec2::new(LEVEL_WIDTH, GROUND_HEIGHT),
        ))
        .insert(Transform::from_translation(GROUND_TRANSLATION));
}

fn move_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera), With<GameCamera>>,
    time: Res<Time>,
) {
    let (mut transform, camera) = query.single_mut().unwrap();
    let camera_width = camera.logical_viewport_size().unwrap().x;
    let camera_half_width = camera_width * 0.5;

    let speed = 500.0;

    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed * time.delta_secs();
    }

    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed * time.delta_secs();
    }

    let min_x = LEVEL_START + camera_half_width;
    let max_x = LEVEL_END - camera_half_width;

    transform.translation.x = transform.translation.x.clamp(min_x, max_x);
}
