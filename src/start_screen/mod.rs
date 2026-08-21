use bevy::prelude::*;

use crate::state::GameState;

pub struct StartScreenPlugin<S: States> {
    pub state: S,
}

impl<S: States> StartScreenPlugin<S> {
    pub fn new(s: S) -> Self {
        Self { state: s }
    }
}

impl<S: States> Plugin for StartScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_buttons.run_if(in_state(self.state.clone())));

        app.init_state::<GameState>();
    }
}

#[derive(Component)]
pub struct StartButton;

fn setup_buttons(mut commands: Commands) {
    let center = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor::from(Color::linear_rgb(0., 0., 1.)),
        ))
        .id();

    let button_container = commands
        .spawn((
            Node {
                width: percent(20),
                flex_direction: FlexDirection::Column,
                column_gap: percent(20),
                padding: UiRect::all(px(4)),
                ..default()
            },
            BackgroundColor::from(Color::WHITE),
        ))
        .id();

    let button_bundle = (
        Node {
            padding: UiRect::all(px(4)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        Button,
        BackgroundColor::from(Color::linear_rgb(1., 0., 0.)),
    );

    let start = commands
        .spawn((
            button_bundle.clone(),
            StartButton,
            children![(
                Text::new("Play"),
                TextColor::WHITE,
                TextFont {
                    font_size: 40.,
                    weight: FontWeight(800),
                    ..default()
                }
            )],
        ))
        .id();

    commands.entity(center).add_child(button_container);
    commands.entity(button_container).add_child(start);
}
