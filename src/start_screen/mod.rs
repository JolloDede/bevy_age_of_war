use bevy::prelude::*;

use crate::state::GameState;

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
        app.add_systems(OnEnter(GameState::StartScreen), setup);
        app.add_systems(Update, button_system);
        app.add_systems(OnExit(GameState::StartScreen), cleanup_menu);

        app.init_state::<GameState>();
    }
}

#[derive(Component)]
pub struct StartScreenMarker;

#[derive(Component)]
pub struct StartButton;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            top: percent(20),
            width: percent(100),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ZIndex(1),
        StartScreenMarker,
        children![
            Node {
                width: percent(80),
                ..default()
            },
            ImageNode::from(asset_server.load("title.png")),
        ],
    ));

    let center = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::from(asset_server.load("Background.png")),
            StartScreenMarker,
        ))
        .id();

    let button_container = commands
        .spawn((
            Node {
                width: percent(40),
                height: percent(60),
                flex_direction: FlexDirection::Column,
                column_gap: percent(20),
                padding: UiRect::all(px(20)),
                margin: UiRect::bottom(px(10)),
                border_radius: BorderRadius::all(percent(20)),
                ..default()
            },
            BackgroundColor::from(Color::linear_rgba(1., 1., 1., 0.2)),
        ))
        .id();

    let button_bundle = (
        Node {
            padding: UiRect::all(px(4)),
            justify_content: JustifyContent::Center,
            column_gap: px(16),
            ..default()
        },
        Button,
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
    startscreen_query: Query<Entity, With<StartScreenMarker>>,
) {
    for entity in startscreen_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct Selector;

pub fn button_system(
    action_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<StartButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selector_query: Query<Entity, With<Selector>>,
) {
    for (interaction, entity) in action_query.iter() {
        match interaction {
            Interaction::Pressed => {
                game_state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                let bla = commands
                    .spawn((
                        ImageNode::from(asset_server.load("sword.png")),
                        UiTransform::from_rotation(Rot2::degrees(90.)),
                        Selector,
                    ))
                    .id();

                commands.entity(entity).insert_child(0, bla);
            }
            Interaction::None => {
                if let Ok(entity) = selector_query.single() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
