use std::sync::Arc;

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    age_of_war::Age,
    consts::{HUD_LAYER, MENU_TEXT, MENU_TEXT_COLOR},
    event::{BaseAdvanceAgeEvent, UnitQueueEvent},
    game_turret::TurretType,
    game_unit::{GameUnit, UnitType},
    hud::BaseAge,
    resource_paths,
};

const ACTION_BUTTON_SIZE: Vec2 = Vec2::new(60., 60.);

#[derive(Component)]
pub enum ButtonGroup {
    Main,
    Units,
    Turrets,
}

#[derive(Component, Clone, Copy)]
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
pub struct UnitButtons(pub UnitType);

#[derive(Component)]
pub struct TurretButtons(pub TurretType);

#[derive(Component, Clone, Copy)]
pub struct ButtonPressed(pub bool);

#[derive(Component, Clone, Copy)]
pub struct FrameButton;

#[derive(Component)]
pub struct MenuText;

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
    let default_node_bundle = (
        default_node.clone(),
        ImageNode::from(asset_server.load("menu/frame.png")),
        FrameButton,
        Button,
        ButtonPressed(false),
    );

    let back_arrow_child = (
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        ImageNode::from(asset_server.load("menu/backarrow.png")),
    );

    commands.spawn((
        MenuText,
        Text::new(MENU_TEXT),
        TextColor(MENU_TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            left: percent(67),
            top: px(40),
            ..default()
        },
        ZIndex(2),
    ));

    let mut button_container = commands.spawn((
        Node {
            width: percent(91),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            margin: UiRect {
                top: px(56),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(HUD_LAYER),
        ZIndex(2),
    ));

    button_container.with_children(|parent| {
        let mut first_row = parent.spawn((
            Node {
                width: percent(100),
                height: px(ACTION_BUTTON_SIZE.y + 10.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                ..default()
            },
            ButtonGroup::Main,
        ));

        first_row.with_children(|row| {
            row.spawn((
                default_node_bundle.clone(),
                MenuNavigationButtons::Unit,
                children![(
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    ImageNode::from(asset_server.load("menu/unit.png"),),
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                MenuNavigationButtons::Turret,
                children![(
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    ImageNode::from(asset_server.load("menu/turret.png"),),
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                MenuActionButton::SelTurret,
                children![(
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    ImageNode::from(asset_server.load("menu/turret.png"),),
                    children![(
                        Node {
                            margin: UiRect::all(px(8.)),
                            ..default()
                        },
                        Text::new("$"),
                        TextColor::from(Color::linear_rgb(0., 1., 0.)),
                        TextFont {
                            font_size: 20.,
                            ..default()
                        },
                    )]
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                MenuActionButton::UpgradeBase,
                children![(
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    ImageNode::from(asset_server.load("menu/turret.png"),),
                    children![(
                        Node {
                            margin: UiRect {
                                left: px(4.),
                                top: px(-2),
                                ..default()
                            },
                            ..default()
                        },
                        Text::new("+"),
                        TextColor::from(Color::linear_rgb(0., 1., 0.)),
                        TextFont {
                            font_size: 40.,
                            ..default()
                        },
                    )]
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                MenuActionButton::AdvanceAge,
                children![(
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    ImageNode::from(asset_server.load("menu/advance.png"),),
                )],
            ));
        });

        let mut second_row = parent.spawn((
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
        ));

        second_row.with_children(|row| {
            row.spawn((
                default_node_bundle.clone(),
                UnitButtons(UnitType::Meele),
                children![(
                    Node {
                        margin: UiRect::left(px(10)),
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    ImageNode::from(asset_server.load(resource_paths::load_unit_buttons(
                        base_age.0,
                        UnitType::Meele,
                    ))),
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                UnitButtons(UnitType::Ranged),
                children![(
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    ImageNode::from(asset_server.load(resource_paths::load_unit_buttons(
                        base_age.0,
                        UnitType::Ranged,
                    ))),
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                UnitButtons(UnitType::Tank),
                children![(
                    Node {
                        width: percent(80),
                        height: percent(80),
                        ..default()
                    },
                    ImageNode::from(asset_server.load(resource_paths::load_unit_buttons(
                        base_age.0,
                        UnitType::Tank,
                    ))),
                )],
            ));
            row.spawn((default_node.clone(),));
            row.spawn((
                default_node_bundle.clone(),
                MenuNavigationButtons::Back,
                children![(back_arrow_child.clone())],
            ));
        });

        let mut third_row = parent.spawn((
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
        ));

        third_row.with_children(|row| {
            row.spawn((
                default_node_bundle.clone(),
                TurretButtons(TurretType::Small),
                children![(
                    Node {
                        width: percent(80),
                        height: percent(80),
                        ..default()
                    },
                    ImageNode::from(asset_server.load(resource_paths::load_turret_buttons(
                        base_age.0,
                        TurretType::Small,
                    ))),
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                TurretButtons(TurretType::Medium),
                children![(
                    Node {
                        width: percent(80),
                        height: percent(80),
                        ..default()
                    },
                    ImageNode::from(asset_server.load(resource_paths::load_turret_buttons(
                        base_age.0,
                        TurretType::Medium,
                    ))),
                )],
            ));
            row.spawn((
                default_node_bundle.clone(),
                TurretButtons(TurretType::Large),
                children![(
                    Node {
                        width: percent(80),
                        height: percent(80),
                        ..default()
                    },
                    ImageNode::from(asset_server.load(resource_paths::load_turret_buttons(
                        base_age.0,
                        TurretType::Large,
                    ))),
                )],
            ));
            row.spawn((default_node.clone(),));
            row.spawn((
                default_node_bundle.clone(),
                MenuNavigationButtons::Back,
                children![(back_arrow_child.clone())],
            ));
        });
    });
}

pub fn menu_navigation_button_system(
    mut action_query: Query<
        (&Interaction, &MenuNavigationButtons, &mut ButtonPressed),
        (Changed<Interaction>, With<Button>),
    >,
    mut button_groups: Query<(&ButtonGroup, &mut Node)>,
    mut menu_text: Single<&mut Text, With<MenuText>>,
) {
    for (interaction, action, mut pressed) in action_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                pressed.0 = true;
            }
            _ => {
                if pressed.0 {
                    pressed.0 = false;
                    match action {
                        MenuNavigationButtons::Unit => {
                            menu_text.0 = "Menu - Units".to_string();
                            for (group, mut node) in &mut button_groups {
                                node.display = match group {
                                    ButtonGroup::Main => Display::None,
                                    ButtonGroup::Units => Display::Flex,
                                    ButtonGroup::Turrets => Display::None,
                                };
                            }
                        }
                        MenuNavigationButtons::Turret => {
                            menu_text.0 = "Menu - Turrets".to_string();
                            for (group, mut node) in &mut button_groups {
                                node.display = match group {
                                    ButtonGroup::Main => Display::None,
                                    ButtonGroup::Units => Display::None,
                                    ButtonGroup::Turrets => Display::Flex,
                                };
                            }
                        }
                        MenuNavigationButtons::Back => {
                            menu_text.0 = MENU_TEXT.to_string();
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
    }
}

pub fn frame_button_system(
    mut action_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<FrameButton>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut sprite) in action_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                sprite.image = asset_server.load("menu/frame_click.png");
            }
            Interaction::Hovered => {
                sprite.image = asset_server.load("menu/frame_hover.png");
            }
            Interaction::None => {
                sprite.image = asset_server.load("menu/frame.png");
            }
        }
    }
}

pub fn main_button_system(
    mut commands: Commands,
    action_query: Query<(&Interaction, &MenuActionButton), (Changed<Interaction>, With<Button>)>,
    mut age: ResMut<BaseAge>,
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
                    age.0 = match age.0 {
                        Age::StoneAge => Age::Medival,
                        Age::Medival => Age::Renaissance,
                        Age::Renaissance => Age::Modern,
                        Age::Modern => Age::Future,
                        Age::Future => panic!("Already reached the last age"),
                    };
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

pub fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ImageNode::from(asset_server.load("hud_banners.png")),
        Node {
            width: percent(100),
            ..default()
        },
        ZIndex(1),
    ));
}
