use std::time::Duration;

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    consts::HUD_LAYER,
    event::{QueueTimerFinishedEvent, UnitQueueEvent, UnitSpawnEvent},
    game_unit::GameUnit,
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

        app.insert_resource(EntityQueue::default());
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
    let entry = queue.get_and_clear_last().unwrap();
    commands.trigger(UnitSpawnEvent(entry));

    // progress the next one
    for mut bar in timer_query.iter_mut() {
        bar.unit = None;
        if let Some(entry) = *queue.get_last() {
            bar.set_unit(entry);
        }
    }
}
