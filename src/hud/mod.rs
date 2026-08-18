use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    age_of_war::Age,
    consts::HUD_LAYER,
    event::{BaseAdvanceAgeEvent, QueueTimerFinishedEvent, UnitSpawnEvent},
    resource_paths,
};

mod menu;
use menu::*;
mod queue;
use queue::*;
mod progressbar;
use progressbar::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);

        app.add_systems(Startup, setup_buttons);
        app.add_systems(Update, menu_navigation_button_system);
        app.add_systems(Update, main_button_system);
        app.add_systems(Update, unit_button_system);
        app.add_systems(Update, turret_button_system);

        app.add_systems(Startup, setup_queue);
        app.add_systems(Update, queue_system);
        app.add_observer(unit_queue_observer);

        app.add_systems(Startup, setup_progressbar);
        app.add_systems(Update, progressbar_system);

        app.add_observer(timer_finished);

        app.add_observer(advance_age_observer);

        app.insert_resource(EntityQueue::default());
        app.insert_resource(BaseAge::default());
    }
}

#[derive(Component)]
struct HudCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..default()
        },
        RenderLayers::layer(HUD_LAYER),
        HudCamera,
    ));
}

fn timer_finished(
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
        Query<(&mut ImageNode, &UnitButtons)>,
        Query<(&mut ImageNode, &TurretButtons)>,
    )>,
    asset_server: Res<AssetServer>,
) {
    debug!("hud advance age event");
    for (mut image_node, unit_type) in button_sets.p0().iter_mut() {
        image_node.image = asset_server.load(resource_paths::load_unit_buttons(age.0, unit_type.0))
    }
    for (mut image_node, turret_type) in button_sets.p1().iter_mut() {
        image_node.image =
            asset_server.load(resource_paths::load_turret_buttons(age.0, turret_type.0))
    }
}
