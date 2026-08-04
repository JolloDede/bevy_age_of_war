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
