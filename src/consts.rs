use bevy::prelude::*;

pub const GAME_LAYER: usize = 0;
pub const HUD_LAYER: usize = 1;

pub const QUEUE_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.);
pub const QUEUE_COLOR_OCCUPIED: Color = Color::linear_rgb(0.5, 0.5, 0.5);
pub const QUEUE_BORDER_COLOR: Color = Color::BLACK;
pub const QUEUE_SIZE: u8 = 5;
pub const QUEUE_RECT_WIDTH: i32 = 20;
pub const QUEUE_RECT_HEIGHT: i32 = 20;
pub const MAX_QUEUE_SIZE: usize = 5;

pub const PROGRESSBAR_HEIGHT: i32 = 10;

pub const LEVEL_SIZE: Vec2 = Vec2::new(1520., 504.);
pub const LEVEL_START: f32 = -(LEVEL_SIZE.x * 0.5);
pub const LEVEL_END: f32 = LEVEL_SIZE.x * 0.5;

pub const BASE_MARGIN: f32 = 0.;
pub const BASE_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
pub const BASE_SIZE: Vec2 = Vec2::new(80.0, 169.0);
pub const BASE_HEALTH_BAR_HEIGHT: f32 = 160.;
pub const BASE_START_HEALTH: i32 = 500;
pub const BASE_EXPAND_SIZE: Vec2 = Vec2::new(51., 50.);
pub const BASE_MAX_TOWER_COUNT: usize = 3;

pub const GROUND_Y: f32 = -(LEVEL_SIZE.y * 0.5) + (GROUND_HEIGHT * 0.8);
pub const GROUND_HEIGHT: f32 = 50.0;

pub const UNIT_COLOR: Color = Color::linear_rgb(1.0, 0.0, 1.0);

pub const MENU_TEXT_COLOR: Color = Color::linear_rgb(1., 1., 0.);
pub const MENU_TEXT: &str = "Menu";
pub const ACTION_BUTTON_SIZE: Vec2 = Vec2::new(60., 60.);
