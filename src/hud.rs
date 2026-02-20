use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, setup_buttons);
    }
}

#[derive(Component)]
struct HudCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), HudCamera));
}

const ACTION_BUTTON_WIDTH: i32 = 60;
const ACTION_BUTTON_HEIGHT: i32 = 60;
const TEMP_BUTTON_FONT: f32 = 12.0;

fn setup_buttons(mut commands: Commands) {
    commands
        .spawn((Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::End,
            align_items: AlignItems::Start,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: px(ACTION_BUTTON_WIDTH),
                    height: px(ACTION_BUTTON_HEIGHT),
                    border: UiRect::all(px(5)),
                    border_radius: BorderRadius::ZERO,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Button,
                BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                children![(
                    Text::new("Units"),
                    TextFont {
                        font_size: TEMP_BUTTON_FONT,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ));

            parent.spawn((
                Node {
                    width: px(ACTION_BUTTON_WIDTH),
                    height: px(ACTION_BUTTON_HEIGHT),
                    border: UiRect::all(px(5)),
                    border_radius: BorderRadius::ZERO,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Button,
                BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                children![(
                    Text::new("Turrets"),
                    TextFont {
                        font_size: TEMP_BUTTON_FONT,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ));

            parent.spawn((
                Node {
                    width: px(ACTION_BUTTON_WIDTH),
                    height: px(ACTION_BUTTON_HEIGHT),
                    border: UiRect::all(px(5)),
                    border_radius: BorderRadius::ZERO,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Button,
                BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                children![(
                    Text::new("Sel Turret"),
                    TextFont {
                        font_size: TEMP_BUTTON_FONT,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ));

            parent.spawn((
                Node {
                    width: px(ACTION_BUTTON_WIDTH),
                    height: px(ACTION_BUTTON_HEIGHT),
                    border: UiRect::all(px(5)),
                    border_radius: BorderRadius::ZERO,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Button,
                BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                children![(
                    Text::new("Advance Age"),
                    TextFont {
                        font_size: TEMP_BUTTON_FONT,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )],
            ));
        });
}
