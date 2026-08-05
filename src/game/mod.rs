use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{consts::GAME_LAYER, event::UnitSpawnEvent};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, spawn_world);
        app.add_systems(Startup, spawn_bases);
        app.add_systems(Update, move_camera);

        app.add_observer(unit_spawn_observer);
    }
}

const LEVEL_WIDTH: f32 = 2000.0;

#[derive(Component)]
struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    let start_x = -(LEVEL_WIDTH * 0.5);

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(start_x, 0.0, 1.0),
        RenderLayers::layer(GAME_LAYER),
        GameCamera,
    ));
}

const GROUND_HEIGHT: f32 = 50.0;
const GROUND_TRANSLATION: Vec3 = Vec3::new(0.0, -100.0, 0.0);
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

#[derive(Component)]
struct Base;

const BASE_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
const BASE_SIZE: Vec2 = Vec2::new(80.0, 120.0);
const GROUND_Y: f32 = GROUND_TRANSLATION.y + (GROUND_HEIGHT * 0.5);
fn spawn_bases(mut commands: Commands) {
    let left_x = -(LEVEL_WIDTH * 0.5) + (BASE_SIZE.x * 0.5);
    let right_x = (LEVEL_WIDTH * 0.5) - (BASE_SIZE.x * 0.5);
    let base_y = GROUND_Y + (BASE_SIZE.y * 0.5);

    commands
        .spawn(Sprite::from_color(BASE_COLOR, BASE_SIZE))
        .insert(Transform::from_xyz(left_x, base_y, 1.0))
        .insert(Base);
    commands
        .spawn(Sprite::from_color(BASE_COLOR, BASE_SIZE))
        .insert(Transform::from_xyz(right_x, base_y, 1.0))
        .insert(Base);
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

    let min_x = -(LEVEL_WIDTH * 0.5) + camera_half_width;
    let max_x = (LEVEL_WIDTH * 0.5) - camera_half_width;

    transform.translation.x = transform.translation.x.clamp(min_x, max_x);
}

fn unit_spawn_observer(spawn_event: On<UnitSpawnEvent>) {
    info!("Unit Spawn Event fired");
}
