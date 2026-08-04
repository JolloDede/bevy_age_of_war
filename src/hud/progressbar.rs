use bevy::prelude::*;

use crate::consts::{QUEUE_MARGIN_LEFT, QUEUE_MARGIN_TOP, QUEUE_RECT_HEIGHT};

#[derive(Component)]
pub struct QueueTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct ProgressbarFill;

const PROGRESSBAR_QUEUE_PADDING: f32 = 2.0;
const PROGRESSBAR_HEIGHT: i32 = 10;

pub fn setup_progressbar(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: px(400.0),
                height: px(PROGRESSBAR_HEIGHT),
                border: UiRect::all(px(2.0)),
                margin: UiRect {
                    top: px(QUEUE_MARGIN_TOP
                        + QUEUE_RECT_HEIGHT as f32
                        + PROGRESSBAR_QUEUE_PADDING),
                    left: px(QUEUE_MARGIN_LEFT),
                    ..Default::default()
                },
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor::all(Color::WHITE),
            QueueTimer {
                timer: Timer::from_seconds(3.0, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.7, 0.3)),
                ProgressbarFill,
            ));
        });
}

pub fn progressbar_system(
    time: Res<Time>,
    mut timer_query: Query<(&mut QueueTimer, &Children)>,
    mut style_query: Query<&mut Node, With<ProgressbarFill>>,
) {
    for (mut bar, children) in timer_query.iter_mut() {
        bar.timer.tick(time.delta());

        let percent = (1.0 - bar.timer.fraction()) * 100.0;
        for &child in children {
            if let Ok(mut node) = style_query.get_mut(child) {
                node.width = Val::Percent(percent);
            }
        }
    }
}
