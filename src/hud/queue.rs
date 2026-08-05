use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    consts::*,
    event::{QueueTimerFinishedEvent, UnitQueueEvent, UnitSpawnEvent},
    game_unit::{GameUnit, QueueUnit},
    hud::progressbar::QueueTimer,
};

#[derive(Default, Resource, Deref)]
pub struct EntityQueue(Box<[Entity]>);

#[derive(Component, Deref, Clone, Copy)]
pub struct QueueEntry(pub Option<GameUnit>);

impl QueueEntry {
    pub fn new() -> Self {
        Self(None)
    }
}

pub fn setup_queue(mut commands: Commands, mut queue: ResMut<EntityQueue>) {
    let default_node = Node {
        width: px(QUEUE_RECT_WIDTH),
        height: px(QUEUE_RECT_HEIGHT),
        border: UiRect::all(px(2)),
        border_radius: BorderRadius::ZERO,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

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
        ))
        .with_children(|row| {
            *queue = EntityQueue(Box::new([
                row.spawn(default_node.clone())
                    .insert(QueueEntry::new())
                    .insert(BorderColor::from(QUEUE_BORDER_COLOR))
                    .insert(BackgroundColor::from(QUEUE_COLOR))
                    .id(),
                row.spawn(default_node.clone())
                    .insert(QueueEntry::new())
                    .insert(BorderColor::from(QUEUE_BORDER_COLOR))
                    .insert(BackgroundColor::from(QUEUE_COLOR))
                    .id(),
                row.spawn(default_node.clone())
                    .insert(QueueEntry::new())
                    .insert(BorderColor::from(QUEUE_BORDER_COLOR))
                    .insert(BackgroundColor::from(QUEUE_COLOR))
                    .id(),
                row.spawn(default_node.clone())
                    .insert(QueueEntry::new())
                    .insert(BorderColor::from(QUEUE_BORDER_COLOR))
                    .insert(BackgroundColor::from(QUEUE_COLOR))
                    .id(),
                row.spawn(default_node.clone())
                    .insert(QueueEntry::new())
                    .insert(BorderColor::from(QUEUE_BORDER_COLOR))
                    .insert(BackgroundColor::from(QUEUE_COLOR))
                    .id(),
            ]));
        });
}

pub fn queue_system(
    queue: ResMut<EntityQueue>,
    mut queue_items: Query<&mut QueueEntry>,
    mut bg_colors: Query<&mut BackgroundColor>,
) {
    for q in queue.0.iter() {
        let entry = *queue_items.get_mut(*q).unwrap();
        let mut bg_color = bg_colors.get_mut(*q).unwrap();
        if entry.0.is_none() {
            *bg_color = BackgroundColor::from(QUEUE_COLOR);
        } else {
            *bg_color = BackgroundColor::from(QUEUE_COLOR_OCCUPIED);
        }
    }
}

pub fn unit_queue_observer(
    unit: On<UnitQueueEvent>,
    mut queue_entries: Query<&mut QueueEntry>,
    mut progress_query: Query<&mut QueueTimer>,
) {
    debug!("Triggered UnitQueueEvent with: {:?}", unit.event());
    let mut queue_iter = queue_entries.iter_mut();
    while let Some(mut item) = queue_iter.next() {
        if item.0.is_none() {
            item.0 = match *unit.event() {
                UnitQueueEvent::Meele => Some(GameUnit::Meele),
                UnitQueueEvent::Ranged => Some(GameUnit::Ranged),
                UnitQueueEvent::Tank => Some(GameUnit::Tank),
                UnitQueueEvent::Super => Some(GameUnit::Super),
            };
            break;
        }
    }

    // add to the progressbar if its empty
    for mut progress in progress_query.iter_mut() {
        if progress.unit.is_none() {
            for item in queue_entries.iter_mut() {
                match item.0 {
                    Some(unit) => {
                        progress.set_unit(unit);
                        break;
                    }
                    None => continue,
                }
            }
        }
    }
}
