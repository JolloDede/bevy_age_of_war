use std::sync::Arc;

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    consts::HUD_LAYER,
    event::{BaseAdvanceAgeEvent, UnitQueueEvent},
    game_turret::TurretType,
    game_unit::{GameUnit, UnitType},
    hud::BaseAge,
    resource_paths,
};

const ACTION_BUTTON_SIZE: Vec2 = Vec2::new(60., 60.);
const TEMP_BUTTON_FONT: f32 = 12.0;

#[derive(Component)]
pub enum ButtonGroup {
    Main,
    Units,
    Turrets,
}

#[derive(Component, Clone)]
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
pub struct TurretButtons(TurretType);

pub fn setup_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    base_age: Res<BaseAge>,
) {
    let default_node = Node {
        width: px(ACTION_BUTTON_SIZE.x),
        height: px(ACTION_BUTTON_SIZE.y),
        border: UiRect::all(px(5)),
        border_radius: BorderRadius::ZERO,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let back_bundle_unit = (
        default_node.clone(),
        ImageNode::from(asset_server.load("iconback.png")),
        MenuNavigationButtons::Back,
        Button,
    );
    let back_bundle_turret = (
        back_bundle_unit.0.clone(),
        back_bundle_unit.1.clone(),
        back_bundle_unit.2.clone(),
        back_bundle_unit.3.clone(),
    );

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
                        height: px(ACTION_BUTTON_SIZE.y + 10.),
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
                        height: px(ACTION_BUTTON_SIZE.y + 10.),
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
                        ImageNode::from(asset_server.load(resource_paths::load_unit_buttons(
                            base_age.0,
                            UnitType::Meele,
                        ))),
                        UnitButtons(UnitType::Meele),
                        Button,
                    ));
                    row.spawn((
                        default_node.clone(),
                        ImageNode::from(asset_server.load(resource_paths::load_unit_buttons(
                            base_age.0,
                            UnitType::Ranged,
                        ))),
                        UnitButtons(UnitType::Ranged),
                        Button,
                    ));
                    row.spawn((
                        default_node.clone(),
                        ImageNode::from(asset_server.load(resource_paths::load_unit_buttons(
                            base_age.0,
                            UnitType::Tank,
                        ))),
                        UnitButtons(UnitType::Tank),
                        Button,
                    ));
                    // row.spawn((
                    //     default_node.clone(),
                    //     UnitButtons(UnitType::Super),
                    //     Button,
                    //     BackgroundColor(Color::linear_rgb(0.0, 0.5, 1.0)),
                    //     children![(
                    //         Text::new("Super Human"),
                    //         TextFont {
                    //             font_size: TEMP_BUTTON_FONT,
                    //             ..default()
                    //         },
                    //         TextColor(Color::WHITE),
                    //     )],
                    // ));
                    row.spawn(back_bundle_unit);
                });

            // Third row
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(ACTION_BUTTON_SIZE.y + 10.),
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
                        ImageNode::from(asset_server.load(resource_paths::load_turret_buttons(
                            base_age.0,
                            TurretType::Small,
                        ))),
                        TurretButtons(TurretType::Small),
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.5)),
                    ));
                    row.spawn((
                        default_node.clone(),
                        TurretButtons(TurretType::Medium),
                        ImageNode::from(asset_server.load(resource_paths::load_turret_buttons(
                            base_age.0,
                            TurretType::Medium,
                        ))),
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.5)),
                    ));
                    row.spawn((
                        default_node.clone(),
                        TurretButtons(TurretType::Large),
                        ImageNode::from(asset_server.load(resource_paths::load_turret_buttons(
                            base_age.0,
                            TurretType::Large,
                        ))),
                        Button,
                        BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.5)),
                    ));
                    row.spawn(back_bundle_turret);
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
            match action.0 {
                TurretType::Small => {
                    println!("Small Turret")
                }
                TurretType::Medium => {
                    println!("Medium Turret")
                }
                TurretType::Large => {
                    println!("Big Turret")
                }
            }
        }
    }
}
