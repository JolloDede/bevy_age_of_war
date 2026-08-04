use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{consts::HUD_LAYER, event::UnitQueueEvent};

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
        app.add_observer(queue_observer);

        app.add_systems(Startup, setup_progressbar);
        app.add_systems(Update, progressbar_system);

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
