use std::sync::Arc;

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    consts::HUD_LAYER,
    event::{BaseAdvanceAgeEvent, UnitQueueEvent},
    game_unit::{GameUnit, UnitType},
    hud::BaseAge,
};

const ACTION_BUTTON_WIDTH: i32 = 60;
const ACTION_BUTTON_HEIGHT: i32 = 60;
const TEMP_BUTTON_FONT: f32 = 12.0;

#[derive(Component)]
pub enum ButtonGroup {
    Main,
    Units,
    Turrets,
}

#[derive(Component)]
pub enum MenuNavigationButtons {
    Unit,
    Turret,
    Back,
}

#[derive(Component)]
pub enum MenuActionButton {
    SelTurret,
    UpgradeBase,
    AdvanceAge,
}

#[derive(Component)]
pub struct UnitButtons(UnitType);

#[derive(Component)]
pub enum TurretButtons {
    Small,
    Medium,
    Big,
}

pub fn setup_buttons(mut commands: Commands) {
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
                        UnitButtons(UnitType::Meele),
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
                        UnitButtons(UnitType::Ranged),
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
                        UnitButtons(UnitType::Tank),
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
                        UnitButtons(UnitType::Super),
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

pub fn menu_navigation_button_system(
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

pub fn main_button_system(
    mut commands: Commands,
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
                    commands.trigger(BaseAdvanceAgeEvent);
                }
            }
        }
    }
}

pub fn unit_button_system(
    mut commands: Commands,
    action_query: Query<(&Interaction, &UnitButtons), (Changed<Interaction>, With<Button>)>,
    base_age: Res<BaseAge>,
) {
    for (interaction, unit_button) in action_query.iter() {
        if *interaction == Interaction::Pressed {
            commands.trigger(UnitQueueEvent(Arc::new(GameUnit::new(
                base_age.0,
                unit_button.0,
            ))));
        }
    }
}
pub fn turret_button_system(
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
