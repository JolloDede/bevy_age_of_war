use std::{collections::VecDeque, sync::Arc};

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{consts::*, event::UnitQueueEvent, game_unit::GameUnit, hud::progressbar::QueueTimer};

#[derive(Resource, Deref)]
pub struct EntityQueue(VecDeque<QueueEntry>);

impl Default for EntityQueue {
    fn default() -> Self {
        let mut entries = Vec::new();
        for _ in 0..5 {
            entries.push(QueueEntry::new());
        }
        Self(VecDeque::from(entries))
    }
}

impl EntityQueue {
    pub fn get_last(&self) -> QueueEntry {
        let res = self.0.front().unwrap();

        for entry in self.0.iter().rev() {
            if entry.0.is_some() {
                return QueueEntry(entry.0.clone());
            }
        }

        return QueueEntry(res.0.clone());
    }

    pub fn get_and_clear_last(&mut self) -> QueueEntry {
        for entry in self.0.iter_mut().rev() {
            if entry.0.is_some() {
                // let res = entry.clone();
                // entry.0 = None;
                return QueueEntry(entry.0.take());
            }
        }

        error!("Failed to get and clear last element of EntityQueue");
        QueueEntry(None)
    }
}

#[derive(Component, Deref)]
pub struct QueueEntry(pub Option<Arc<GameUnit>>);

impl QueueEntry {
    pub fn new() -> Self {
        Self(None)
    }
}

#[derive(Component)]
pub struct QueueRowMarker;

const MAX_QUEUE_SIZE: usize = 5;

#[derive(Component, Deref)]
pub struct QueueIndex(usize);

pub fn setup_queue(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                column_gap: px(2),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                margin: UiRect {
                    top: (px(QUEUE_MARGIN_TOP)),
                    left: (px(QUEUE_MARGIN_LEFT)),
                    ..Default::default()
                },
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
        });
}

pub fn queue_system(
    queue: ResMut<EntityQueue>,
    mut index_query: Query<(&QueueIndex, &mut BackgroundColor)>,
) {
    if !queue.is_changed() {
        return;
    }

    for (q_index, mut bg_color) in index_query.iter_mut() {
        let item = queue.get(q_index.0).unwrap();
        *bg_color = match item.0 {
            Some(_) => BackgroundColor::from(QUEUE_COLOR_OCCUPIED),
            None => BackgroundColor::from(QUEUE_COLOR),
        };
    }
}

pub fn unit_queue_observer(
    unit: On<UnitQueueEvent>,
    mut queue: ResMut<EntityQueue>,
    mut progress_query: Query<&mut QueueTimer>,
) {
    debug!("Triggered UnitQueueEvent with: {:?}", unit.event());

    queue.0.push_front(QueueEntry(Some(unit.0.clone())));

    for mut progress in progress_query.iter_mut() {
        if progress.unit.is_none() {
            let unit = queue.get_last().0.unwrap();
            progress.set_unit(unit);
        }
    }
}
