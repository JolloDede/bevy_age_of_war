use std::sync::Arc;

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    age_of_war::Age,
    consts::{HUD_LAYER, MENU_TEXT, MENU_TEXT_COLOR},
    event::{BaseAdvanceAgeEvent, UnitQueueEvent},
    game_turret::TurretType,
    game_unit::{GameUnit, UnitType},
    hud::{
        BaseAge, MenuActionButton, MenuNavigationButton,
        menu::{
            component::{arrow_node, place_holder},
            main_buttons::{action_buttons, navigation_buttons},
            turret_buttons::turret_button_spawner,
            unit_buttons::unit_button_spawner,
        },
    },
};

mod component;
mod main_buttons;
mod turret_buttons;
mod unit_buttons;

#[derive(Component, Debug)]
pub enum ButtonGroup {
    Main,
    Units,
    Turrets,
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

#[derive(Component)]
pub struct PlaceHolderComp;

pub fn setup_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    base_age: Res<BaseAge>,
) {
    let menu_container = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: percent(67),
                top: px(30),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                ..default()
            },
            RenderLayers::layer(HUD_LAYER),
            ZIndex(2),
        ))
        .id();

    let menu_text = commands
        .spawn((MenuText, Text::new(MENU_TEXT), TextColor(MENU_TEXT_COLOR)))
        .id();

    let button_container = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },))
        .id();

    commands
        .entity(menu_container)
        .add_children(&[menu_text, button_container]);

    let mut main_buttons = Vec::with_capacity(5);

    main_buttons.append(&mut navigation_buttons(&mut commands, &asset_server));
    main_buttons.append(&mut action_buttons(&mut commands, &asset_server));

    let main = commands.spawn((Node::DEFAULT, ButtonGroup::Main)).id();
    commands.entity(main).add_children(&main_buttons);
    commands.entity(button_container).add_child(main);

    // Units
    let mut unit_buttons = Vec::with_capacity(5);

    unit_buttons.append(&mut unit_button_spawner(
        &mut commands,
        &asset_server,
        base_age.0,
    ));
    unit_buttons.push(arrow_node(&mut commands, &asset_server));

    let unit = commands
        .spawn((
            Node {
                display: Display::None,
                ..default()
            },
            ButtonGroup::Units,
        ))
        .id();
    commands.entity(unit).add_children(&unit_buttons);
    commands.entity(button_container).add_child(unit);

    // Turrets
    let mut turret_buttons = Vec::with_capacity(5);

    turret_buttons.append(&mut turret_button_spawner(
        &mut commands,
        &asset_server,
        base_age.0,
    ));
    turret_buttons.push(place_holder::<PlaceHolderComp>(&mut commands, None));
    turret_buttons.push(arrow_node(&mut commands, &asset_server));

    let unit = commands
        .spawn((
            Node {
                display: Display::None,
                ..default()
            },
            ButtonGroup::Turrets,
        ))
        .id();
    commands.entity(unit).add_children(&turret_buttons);
    commands.entity(button_container).add_child(unit);
}

pub fn menu_navigation_button_system(
    mut action_query: Query<
        (&Interaction, &MenuNavigationButton, &mut ButtonPressed),
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
                        MenuNavigationButton::Unit => {
                            menu_text.0 = "Menu - Units".to_string();
                            for (group, mut node) in &mut button_groups {
                                debug!("Found any buttongroups {:?} {:?}", group, node.display);
                                node.display = match group {
                                    ButtonGroup::Main => Display::None,
                                    ButtonGroup::Units => Display::Flex,
                                    ButtonGroup::Turrets => Display::None,
                                };
                            }
                        }
                        MenuNavigationButton::Turret => {
                            menu_text.0 = "Menu - Turrets".to_string();
                            for (group, mut node) in &mut button_groups {
                                node.display = match group {
                                    ButtonGroup::Main => Display::None,
                                    ButtonGroup::Units => Display::None,
                                    ButtonGroup::Turrets => Display::Flex,
                                };
                            }
                        }
                        MenuNavigationButton::SelTurret => {
                            todo!()
                        }
                        MenuNavigationButton::Back => {
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
