use bevy::{camera::visibility::RenderLayers, prelude::*};
use strum::EnumIter;

use crate::{
    age_of_war::Age,
    consts::*,
    event::{BaseAdvanceAgeEvent, QueueTimerFinishedEvent, UnitSpawnEvent},
    player::{Experience, Money},
    resource_paths,
    state::GameState,
};

mod menu;
use menu::*;
mod queue;
use queue::*;
mod progressbar;
use progressbar::*;

pub struct HudPlugin<S: States> {
    pub state: S,
}

impl<S: States> HudPlugin<S> {
    pub fn new(s: S) -> Self {
        Self { state: s }
    }
}

#[derive(Component)]
pub struct HudMarker;

impl<S: States> Plugin for HudPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        app.add_systems(OnExit(GameState::InGame), despawn_hud);

        app.add_systems(
            Update,
            collectable_system.run_if(in_state(self.state.clone())),
        );

        app.add_systems(OnEnter(GameState::InGame), setup_buttons);
        app.add_systems(
            Update,
            (
                menu_navigation_button_system.before(frame_button_system),
                main_button_system,
                unit_button_system,
                turret_button_system,
                frame_button_system,
            )
                .run_if(in_state(self.state.clone())),
        );
        app.add_observer(advance_age_observer);

        app.add_systems(OnEnter(GameState::InGame), setup_unit_training);
        app.add_systems(
            Update,
            (queue_system, progressbar_system).run_if(in_state(self.state.clone())),
        );
        app.add_observer(unit_queue_handler);
        app.add_observer(progressbar_finished);

        app.insert_resource(EntityQueue::default());
        app.insert_resource(BaseAge::default());
        app.insert_resource(Money(175));
        app.insert_resource(Experience(0));
    }
}

fn progressbar_finished(
    _timer: On<QueueTimerFinishedEvent>,
    mut commands: Commands,
    mut queue: ResMut<EntityQueue>,
    mut timer_query: Query<&mut QueueTimer>,
) {
    let entry = queue.get_and_clear_last().0.unwrap();
    commands.trigger(UnitSpawnEvent(entry));

    // progress the next one
    for mut bar in timer_query.iter_mut() {
        bar.unit = None;
        if let Some(entry) = queue.get_last().0 {
            bar.set_unit(entry);
        }
    }
}

#[derive(Resource, Deref)]
pub struct BaseAge(pub Age);

impl Default for BaseAge {
    fn default() -> Self {
        Self(Age::StoneAge)
    }
}

pub fn advance_age_observer(
    _advance_event: On<BaseAdvanceAgeEvent>,
    age: Res<BaseAge>,
    mut button_sets: ParamSet<(
        Query<(&UnitButtons, &Children)>,
        Query<(&TurretButtons, &Children)>,
    )>,
    mut child_query: Query<&mut ImageNode>,
    asset_server: Res<AssetServer>,
) {
    debug!("hud advance age event");
    for (unit_type, children) in button_sets.p0().iter_mut() {
        for &child in children {
            if let Ok(mut sprite) = child_query.get_mut(child) {
                sprite.image =
                    asset_server.load(resource_paths::load_unit_buttons(age.0, unit_type.0))
            }
        }
    }
    for (turret_type, children) in button_sets.p1().iter_mut() {
        for &child in children {
            if let Ok(mut sprite) = child_query.get_mut(child) {
                sprite.image =
                    asset_server.load(resource_paths::load_turret_buttons(age.0, turret_type.0))
            }
        }
    }
}

#[derive(Component)]
pub struct HelpText;

fn setup_unit_training(mut commands: Commands) {
    let container = commands
        .spawn((
            Node {
                width: percent(43),
                height: px(20),
                position_type: PositionType::Absolute,
                left: percent(20),
                top: percent(2),
                flex_direction: FlexDirection::Column,
                row_gap: px(4),
                ..default()
            },
            BackgroundColor::from(Color::linear_rgb(1., 0., 0.)),
            ZIndex(3),
            HudMarker,
        ))
        .id();

    let training_container = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(4),
            align_items: AlignItems::Center,
            ..default()
        })
        .id();

    let progress = commands
        .spawn((
            Node {
                height: px(PROGRESSBAR_HEIGHT),
                border: UiRect::all(px(2.0)),
                flex_grow: 1.,
                ..default()
            },
            ZIndex(3),
            BackgroundColor(Color::linear_rgba(0., 0., 0., 0.)),
            BorderColor::all(Color::BLACK),
            QueueTimer {
                timer: Timer::from_seconds(0.0, TimerMode::Once),
                unit: None,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(1.0, 0., 0.)),
                ProgressbarFill,
            ));
        })
        .id();

    let queue = commands
        .spawn((
            Node {
                column_gap: px(2),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            RenderLayers::layer(HUD_LAYER),
            QueueRowMarker,
        ))
        .with_children(|parent| {
            for i in 0..MAX_QUEUE_SIZE {
                parent.spawn((
                    Node {
                        width: px(QUEUE_RECT_WIDTH),
                        height: px(QUEUE_RECT_HEIGHT),
                        border: UiRect::all(px(2)),
                        border_radius: BorderRadius::ZERO,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::from(QUEUE_BORDER_COLOR),
                    BackgroundColor::from(QUEUE_COLOR),
                    QueueIndex(i),
                ));
            }
        })
        .id();

    let help_text = commands
        .spawn((Text::from("Help text"), TextColor::BLACK, HelpText))
        .id();

    commands.entity(container).add_child(training_container);
    commands.entity(container).add_child(help_text);
    commands.entity(training_container).add_child(progress);
    commands.entity(training_container).add_child(queue);
}

#[derive(Component, EnumIter, Clone, Copy, PartialEq)]
pub enum MenuNavigationButton {
    Unit,
    Turret,
    SelTurret,
    Back,
}

#[derive(Component, EnumIter, Clone, Copy, PartialEq)]
pub enum MenuActionButton {
    UpgradeBase,
    AdvanceAge,
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HudMarker>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn();
    }
}
