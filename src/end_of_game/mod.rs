use bevy::prelude::*;

use crate::state::GameState;

pub struct EndOfGamePlugin<S: States> {
    _state: S,
}

impl<S: States> EndOfGamePlugin<S> {
    pub fn new(s: S) -> Self {
        Self { _state: s }
    }
}

impl<S: States> Plugin for EndOfGamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EndOfGame), setup);
        app.add_systems(Update, button_system);
        app.add_systems(OnExit(GameState::EndOfGame), cleanup_menu);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            top: percent(20),
            width: percent(100),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(20),
            ..default()
        },
        ZIndex(1),
        EndScreenMarker,
        children![
            (Text::new("End of game")),
            (
                Node {
                    border: UiRect::all(px(2)),
                    padding: UiRect::all(px(4)),
                    ..default()
                },
                Button,
                BackToMenuButton,
                BorderColor::from(Color::srgb_u8(200, 0, 0)),
                children![Text::new("Go back to Menu")]
            )
        ],
    ));
}

#[derive(Component)]
pub struct BackToMenuButton;

pub fn button_system(
    action_query: Query<&Interaction, (Changed<Interaction>, With<BackToMenuButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in action_query.iter() {
        match interaction {
            Interaction::Pressed => {
                game_state.set(GameState::StartScreen);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Component)]
pub struct EndScreenMarker;

pub fn cleanup_menu(mut commands: Commands, endscreen_query: Query<Entity, With<EndScreenMarker>>) {
    for entity in endscreen_query.iter() {
        commands.entity(entity).despawn();
    }
}
