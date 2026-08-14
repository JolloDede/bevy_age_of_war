use bevy::prelude::*;

pub const GAME_LAYER: usize = 0;
pub const HUD_LAYER: usize = 1;

pub const QUEUE_COLOR: Color = Color::linear_rgb(0.77, 0.77, 0.77);
pub const QUEUE_COLOR_OCCUPIED: Color = Color::linear_rgb(0.0, 1.0, 0.0);
pub const QUEUE_BORDER_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);
// Temp for testing
// pub  const QUEUE_BORDER_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
pub const QUEUE_RECT_WIDTH: i32 = 20;
pub const QUEUE_RECT_HEIGHT: i32 = 20;
pub const QUEUE_MARGIN_TOP: f32 = 20.0;
pub const QUEUE_MARGIN_LEFT: f32 = 20.0;

pub const LEVEL_SIZE: Vec2 = Vec2::new(1510., 504.);
pub const LEVEL_START: f32 = -(LEVEL_SIZE.x * 0.5);
pub const LEVEL_END: f32 = LEVEL_SIZE.x * 0.5;

pub const BASE_MARGIN: f32 = 20.;
pub const BASE_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
pub const BASE_SIZE: Vec2 = Vec2::new(80.0, 169.0);
pub const UNIT_BASE_HEALTH: i32 = 2000;

// pub const GROUND_Y: f32 = GROUND_TRANSLATION.y + (GROUND_HEIGHT * 0.5);
// pub const GROUND_HEIGHT: f32 = 50.0;
// pub const GROUND_TRANSLATION: Vec3 = Vec3::new(0.0, -100.0, 0.0);
pub const GROUND_Y: f32 = -(LEVEL_SIZE.y * 0.5) + GROUND_HEIGHT;
pub const GROUND_HEIGHT: f32 = 50.0;
pub const GROUND_TRANSLATION: Vec3 = Vec3::new(0.0, -100.0, 0.0);

pub const UNIT_SIZE: Vec2 = Vec2::new(20.0, 20.0);
pub const UNIT_COLOR: Color = Color::linear_rgb(1.0, 0.0, 1.0);
