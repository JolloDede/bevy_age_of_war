use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{consts::HUD_LAYER, event::UnitSpawnEvent};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, setup_buttons);
        app.add_systems(Startup, setup_queue);
        app.add_systems(Update, menu_navigation_button_system);
        app.add_systems(Update, main_button_system);
        app.add_systems(Update, unit_button_system);
        app.add_systems(Update, turret_button_system);
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

const ACTION_BUTTON_WIDTH: i32 = 60;
const ACTION_BUTTON_HEIGHT: i32 = 60;
const TEMP_BUTTON_FONT: f32 = 12.0;

#[derive(Component)]
enum ButtonGroup {
    Main,
    Units,
    Turrets,
}

#[derive(Component)]
enum MenuNavigationButtons {
    Unit,
    Turret,
    Back,
}

#[derive(Component)]
enum MenuActionButton {
    SelTurret,
    UpgradeBase,
    AdvanceAge,
}

#[derive(Component)]
enum UnitButtons {
    Meele,
    Ranged,
    Tank,
    Super,
}

#[derive(Component)]
enum TurretButtons {
    Small,
    Medium,
    Big,
}

fn setup_buttons(mut commands: Commands) {
    let default_node = Node {
        width: px(ACTION_BUTTON_WIDTH),
        height: px(ACTION_BUTTON_HEIGHT),
        border: UiRect::all(px(5)),
        border_radius: BorderRadius::ZERO,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    // Root vertical container
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            RenderLayers::layer(HUD_LAYER),
        ))
        .with_children(|parent| {
            // First row
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(ACTION_BUTTON_HEIGHT + 10),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ButtonGroup::Main,
                ))
                .with_children(|row| {
                    row.spawn((
                        default_node.clone(),
                        MenuNavigationButtons::Unit,
                        Button,
                        BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                        children![(
                            Text::new("Units"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        MenuNavigationButtons::Turret,
                        Button,
                        BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                        children![(
                            Text::new("Turrets"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        MenuActionButton::SelTurret,
                        Button,
                        BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                        children![(
                            Text::new("Sel Turret"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        MenuActionButton::UpgradeBase,
                        Button,
                        BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                        children![(
                            Text::new("Upgrade base"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        MenuActionButton::AdvanceAge,
                        Button,
                        BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                        children![(
                            Text::new("Advance Age"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                });

            // Second row
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(ACTION_BUTTON_HEIGHT + 10),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        display: Display::None,
                        ..default()
                    },
                    ButtonGroup::Units,
                ))
                .with_children(|row| {
                    row.spawn((
                        default_node.clone(),
                        UnitButtons::Meele,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 0.5, 1.0)),
                        children![(
                            Text::new("Meele"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        UnitButtons::Ranged,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 0.5, 1.0)),
                        children![(
                            Text::new("Ranged"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        UnitButtons::Tank,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 0.5, 1.0)),
                        children![(
                            Text::new("Tank"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        UnitButtons::Super,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 0.5, 1.0)),
                        children![(
                            Text::new("Super Human"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        MenuNavigationButtons::Back,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 0.5, 1.0)),
                        children![(
                            Text::new("Back"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                });

            // Third row
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(ACTION_BUTTON_HEIGHT + 10),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        display: Display::None,
                        ..default()
                    },
                    ButtonGroup::Turrets,
                ))
                .with_children(|row| {
                    row.spawn((
                        default_node.clone(),
                        TurretButtons::Small,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.5)),
                        children![(
                            Text::new("Small"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        TurretButtons::Medium,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.5)),
                        children![(
                            Text::new("Medium"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        TurretButtons::Big,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.5)),
                        children![(
                            Text::new("Big"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                    row.spawn((
                        default_node.clone(),
                        MenuNavigationButtons::Back,
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 0.5, 1.0)),
                        children![(
                            Text::new("Back"),
                            TextFont {
                                font_size: TEMP_BUTTON_FONT,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                });
        });
}

const QUEUE_COLOR: Color = Color::linear_rgb(0.6, 0.6, 0.0);
const QUEUE_RECT_WIDTH: i32 = 20;
const QUEUE_RECT_HEIGHT: i32 = 20;
fn setup_queue(mut commands: Commands) {
    let queue_size = Vec2::new(10.0, 10.0);
    let default_node = Node {
        width: px(QUEUE_RECT_WIDTH),
        height: px(QUEUE_RECT_HEIGHT),
        border: UiRect::all(px(5)),
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
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            RenderLayers::layer(HUD_LAYER),
        ))
        .with_children(|parent| {
            // First row
            parent.spawn(Sprite::from_color(QUEUE_COLOR, queue_size));
        });
}

fn menu_navigation_button_system(
    action_query: Query<
        (&Interaction, &MenuNavigationButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut button_groups: Query<(&ButtonGroup, &mut Node)>,
) {
    for (interaction, action) in action_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuNavigationButtons::Unit => {
                    for (group, mut node) in &mut button_groups {
                        node.display = match group {
                            ButtonGroup::Main => Display::None,
                            ButtonGroup::Units => Display::Flex,
                            ButtonGroup::Turrets => Display::None,
                        };
                    }
                }
                MenuNavigationButtons::Turret => {
                    for (group, mut node) in &mut button_groups {
                        node.display = match group {
                            ButtonGroup::Main => Display::None,
                            ButtonGroup::Units => Display::None,
                            ButtonGroup::Turrets => Display::Flex,
                        };
                    }
                }
                MenuNavigationButtons::Back => {
                    for (group, mut node) in &mut button_groups {
                        node.display = match group {
                            ButtonGroup::Main => Display::Flex,
                            ButtonGroup::Units => Display::None,
                            ButtonGroup::Turrets => Display::None,
                        };
                    }
                }
            }
        }
    }
}

fn main_button_system(
    action_query: Query<(&Interaction, &MenuActionButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in action_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                MenuActionButton::SelTurret => {
                    println!("Sel turret")
                }
                MenuActionButton::UpgradeBase => {
                    println!("Upgrade base")
                }
                MenuActionButton::AdvanceAge => {
                    println!("advance age")
                }
            }
        }
    }
}

fn unit_button_system(
    mut commands: Commands,
    action_query: Query<(&Interaction, &UnitButtons), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in action_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                UnitButtons::Meele => {
                    dbg!("Trigger meele event");
                    commands.trigger(UnitSpawnEvent::Meele);
                }
                UnitButtons::Ranged => {
                    dbg!("Trigger ranged event");
                    commands.trigger(UnitSpawnEvent::Ranged);
                }
                UnitButtons::Tank => {
                    dbg!("Trigger tank event");
                    commands.trigger(UnitSpawnEvent::Tank);
                }
                UnitButtons::Super => {
                    dbg!("Trigger super event");
                    commands.trigger(UnitSpawnEvent::Super);
                }
            }
        }
    }
}
fn turret_button_system(
    action_query: Query<(&Interaction, &TurretButtons), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in action_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                TurretButtons::Small => {
                    println!("Small Turret")
                }
                TurretButtons::Medium => {
                    println!("Medium Turret")
                }
                TurretButtons::Big => {
                    println!("Big Turret")
                }
            }
        }
    }
}
