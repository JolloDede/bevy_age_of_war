use bevy::prelude::*;

use crate::game::unit::Health;

const HEALTH_BAR_SIZE: Vec2 = Vec2::new(200., 12.);

pub fn health_bar_node() -> (Node, BackgroundColor, BorderColor, Health) {
    let bundle = (
        Node {
            width: px(HEALTH_BAR_SIZE.x),
            height: px(HEALTH_BAR_SIZE.y),
            border: UiRect::all(px(2.0)),
            // margin: UiRect {
            //     top: px(QUEUE_MARGIN_TOP + QUEUE_RECT_HEIGHT as f32 + PROGRESSBAR_QUEUE_PADDING),
            //     left: px(QUEUE_MARGIN_LEFT),
            //     ..Default::default()
            // },
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        BorderColor::all(Color::WHITE),
        Health(32),
    );

    bundle
    // bundle.with_children
    // .with_children(|parent| {
    //     parent.spawn((
    //         Node {
    //             width: Val::Percent(0.0),
    //             height: Val::Percent(100.0),
    //             ..default()
    //         },
    //         BackgroundColor(Color::srgb(0.0, 0.7, 0.3)),
    //         ProgressbarFill,
    //     ));
    // }))
}
