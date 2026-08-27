use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{Base, consts::*, game_unit::UnitType, state::GameState};

mod base;
use base::*;
mod unit;
use unit::*;
mod combat;
use combat::*;
mod health_bar;
use health_bar::*;

pub struct GamePlugin<S: States> {
    pub state: S,
}

impl<S: States> GamePlugin<S> {
    pub fn new(s: S) -> Self {
        Self { state: s }
    }
}

#[derive(Component)]
pub struct GameMarker;

impl<S: States> Plugin for GamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(OnExit(GameState::InGame), despawn_game);

        app.add_systems(
            Update,
            (move_camera, health_system, base_health_text).run_if(in_state(self.state.clone())),
        );

        app.add_systems(OnEnter(GameState::InGame), load_audio);

        app.add_systems(OnEnter(GameState::InGame), spawn_world);

        app.add_systems(OnEnter(GameState::InGame), spawn_bases);
        app.add_observer(advance_age_observer);
        app.add_observer(upgrade_base_observer);

        app.add_systems(
            Update,
            (
                enemy_base_spawn_unit,
                combat_system,
                unit_movement_system.before(clear_unit_collision),
                // Collision
                clear_unit_collision,
                unit_collision_system.after(clear_unit_collision),
                base_collision_system.after(clear_unit_collision),
            )
                .run_if(in_state(self.state.clone())),
        );
        app.add_observer(unit_spawn_observer);

        // Debug
        #[cfg(debug_assertions)]
        {
            app.add_systems(
                Update,
                (
                    draw_rect_around_base,
                    draw_rect_around_units.after(unit_movement_system),
                    draw_attack_ranges.after(unit_movement_system),
                )
                    .run_if(in_state(self.state.clone())),
            );
        }
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
        GameMarker,
    ));
}

fn move_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Single<(&mut Transform, &Camera), With<GameCamera>>,
    time: Res<Time>,
) {
    let (mut transform, camera) = query.into_inner();
    let Some(lvps) = camera.logical_target_size() else {
        return;
    };
    let camera_width = lvps.x;
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

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("Background.png")),
        GameMarker,
    ));
}

pub fn draw_rect_around_units(
    mut gizmos: Gizmos,
    unit_query: Query<(&Transform, &Sprite, &HitBoxSize), With<UnitComp>>,
    images: Res<Assets<Image>>,
) {
    for (trans, sprite, hitbox) in unit_query.iter() {
        let Some(image) = images.get(&sprite.image) else {
            continue;
        };
        let pos = Vec2::new(
            trans.translation.x,
            trans.translation.y + (image.height() as f32 / 4.),
        );

        gizmos.rect_2d(pos, hitbox.0, UNIT_COLOR);
    }
}

fn draw_rect_around_base(
    mut gizmos: Gizmos,
    base_query: Query<(&Transform, &HitBoxSize), With<Base>>,
) {
    for (trans, hitbox) in base_query.iter() {
        let pos = Vec2::new(trans.translation.x, trans.translation.y);

        gizmos.rect_2d(pos, hitbox.0, BASE_COLOR);
    }
}

pub fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("Age of War - Theme Song.mp3")),
        PlaybackSettings::LOOP,
        GameMarker,
    ));
}

#[derive(Component, Deref)]
pub struct HitBoxSize(pub Vec2);

impl From<UnitType> for HitBoxSize {
    fn from(value: UnitType) -> Self {
        Self(match value {
            UnitType::Meele => Vec2::new(40., 80.),
            UnitType::Ranged => Vec2::new(30., 80.),
            UnitType::Tank => Vec2::new(100., 120.),
            UnitType::Super => Vec2::new(30., 80.),
        })
    }
}

impl HitBoxSize {
    pub fn new_base() -> Self {
        Self(Vec2::new(220., 200.))
    }
}

pub fn despawn_game(mut commands: Commands, game_query: Query<Entity, With<GameMarker>>) {
    for entity in game_query.iter() {
        commands.entity(entity).despawn();
    }
}
