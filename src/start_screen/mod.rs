use bevy::prelude::*;

use crate::{hud::HudMarker, state::GameState};

pub struct StartScreenPlugin<S: States> {
    _state: S,
}

impl<S: States> StartScreenPlugin<S> {
    pub fn new(s: S) -> Self {
        Self { _state: s }
    }
}

impl<S: States> Plugin for StartScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartScreen), setup_buttons);
        app.add_systems(Update, button_system);
        app.add_systems(OnExit(GameState::StartScreen), cleanup_menu);

        app.init_state::<GameState>();
    }
}

#[derive(Component)]
pub struct StartScreenMarker;

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
            StartScreenMarker,
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

pub fn cleanup_menu(
    mut commands: Commands,
    startscreen_query: Single<Entity, With<StartScreenMarker>>,
) {
    commands.entity(startscreen_query.entity()).despawn();
}

pub fn button_system(
    action_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in action_query.iter() {
        match interaction {
            Interaction::Pressed => {
                game_state.set(GameState::InGame);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
