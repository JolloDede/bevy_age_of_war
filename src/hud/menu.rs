use std::sync::Arc;

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    CursorMarker,
    age_of_war::Age,
    consts::{HUD_LAYER, MENU_TEXT, MENU_TEXT_COLOR, TURRET_SPRITE_OFFSET},
    event::{
        BaseAdvanceAgeEvent, MarkTurretSpotsEvent, UnMarkTurretSpotsEvent, UnitQueueEvent,
        UpgradeBaseEvent,
    },
    game_turret::{BaseTower, TurretType},
    game_unit::{GameUnit, UnitType},
    hud::{
        BaseAge, HelpText, HudMarker, MenuActionButton, MenuNavigationButton,
        menu::{
            component::{arrow_node, place_holder},
            main_buttons::{action_buttons, navigation_buttons},
            turret_buttons::turret_button_spawner,
            unit_buttons::unit_button_spawner,
        },
    },
    name::{turret_name, unit_name},
    player::{Experience, Money},
    resource_paths,
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
    commands.spawn((
        CursorMarker,
        Transform::from_xyz(TURRET_SPRITE_OFFSET.x, TURRET_SPRITE_OFFSET.y, 4.),
    ));

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
            HudMarker,
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

    let button_node = Node {
        flex_direction: FlexDirection::Row,
        column_gap: px(4),
        ..default()
    };

    commands
        .entity(menu_container)
        .add_children(&[menu_text, button_container]);

    let mut main_buttons = Vec::with_capacity(5);

    main_buttons.append(&mut navigation_buttons(&mut commands, &asset_server));
    main_buttons.append(&mut action_buttons(&mut commands, &asset_server));

    let main = commands
        .spawn((button_node.clone(), ButtonGroup::Main))
        .id();
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
                ..button_node.clone()
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
                ..button_node.clone()
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
    q_cursor: Single<Entity, With<CursorMarker>>,
    mut commands: Commands,
) {
    let cursor_entity = q_cursor.into_inner();
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
                            commands.trigger(UnMarkTurretSpotsEvent);
                            commands.entity(cursor_entity).remove::<Sprite>();
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
    help_text_query: Single<(&mut Text, &mut TextColor), With<HelpText>>,
    money: Res<Money>,
    turret_query: Query<&BaseTower>,
) {
    let (mut text, mut color) = help_text_query.into_inner();
    for (interaction, action) in action_query.iter() {
        match *interaction {
            Interaction::Pressed => match action {
                MenuActionButton::UpgradeBase => {
                    commands.trigger(UpgradeBaseEvent);
                }
                MenuActionButton::AdvanceAge => {
                    age.0 = match age.0 {
                        Age::StoneAge => Age::Medival,
                        Age::Medival => Age::Renaissance,
                        Age::Renaissance => Age::Modern,
                        Age::Modern => Age::Future,
                        Age::Future => {
                            info!("Already reached the last age");
                            Age::Future
                        }
                    };
                    commands.trigger(BaseAdvanceAgeEvent);
                }
            },
            Interaction::Hovered => match action {
                MenuActionButton::UpgradeBase => {
                    let amount = money.tower_upgrade(turret_query.count());
                    text.0 = format!("{}$ - Add a turret spot", amount);
                    color.0 = MENU_TEXT_COLOR;
                }
                MenuActionButton::AdvanceAge => {
                    text.0 = "Cant advance anymore".to_string();
                }
            },
            Interaction::None => {
                text.0.clear();
            }
        }
    }
}

pub fn unit_button_system(
    mut commands: Commands,
    action_query: Query<(&Interaction, &UnitButtons), (Changed<Interaction>, With<Button>)>,
    base_age: Res<BaseAge>,
    mut money: ResMut<Money>,
    help_text_query: Single<(&mut Text, &mut TextColor), With<HelpText>>,
) {
    let (mut text, mut color) = help_text_query.into_inner();

    for (interaction, unit_button) in action_query.iter() {
        let unit_cost = money.unit_price(unit_button.0, base_age.0);

        match *interaction {
            Interaction::Pressed => {
                if money.subtract_money(unit_cost) {
                    commands.trigger(UnitQueueEvent(Arc::new(GameUnit::new(
                        base_age.0,
                        unit_button.0,
                    ))));
                }
            }
            Interaction::Hovered => {
                text.0 = format!("{}$ - {}", unit_cost, unit_name(unit_button.0, base_age.0));
                color.0 = Color::linear_rgb(1., 1., 0.);
            }
            Interaction::None => {
                text.0.clear();
            }
        }
    }
}

pub fn turret_button_system(
    action_query: Query<(&Interaction, &TurretButtons), (Changed<Interaction>, With<Button>)>,
    base_age: Res<BaseAge>,
    money: Res<Money>,
    help_text_query: Single<(&mut Text, &mut TextColor), With<HelpText>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_cursor: Single<Entity, With<CursorMarker>>,
) {
    let (mut text, mut color) = help_text_query.into_inner();
    let cursor_entity = q_cursor.into_inner();
    for (interaction, action) in action_query.iter() {
        let turret_cost = money.tower_price(action.0, base_age.0);
        match *interaction {
            Interaction::Pressed => {
                commands.trigger(MarkTurretSpotsEvent);

                let turret_sprite = resource_paths::load_turret(action.0, base_age.0);
                commands
                    .entity(cursor_entity)
                    .insert(Sprite::from(asset_server.load(turret_sprite)));
            }
            Interaction::Hovered => {
                text.0 = format!("{}$ - {}", turret_cost, turret_name(action.0, base_age.0));
                color.0 = Color::linear_rgb(1., 1., 0.);
            }
            Interaction::None => {
                text.0.clear();
            }
        }
    }
}

#[derive(Component)]
pub struct MoneyComp;

#[derive(Component)]
pub struct ExperienceComp;

pub fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    money: Res<Money>,
    experience: Res<Experience>,
) {
    commands.spawn((
        ImageNode::from(asset_server.load("hud_banners.png")),
        Node {
            width: percent(100),
            ..default()
        },
        ZIndex(1),
        HudMarker,
    ));

    let container = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: percent(6),
                left: percent(4),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ZIndex(2),
        ))
        .id();

    let gold = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: px(10),
                ..default()
            },
            children![
                (ImageNode::from(asset_server.load("menu/coin.png"))),
                (
                    Text::new(money.0.to_string()),
                    TextColor::from(Color::linear_rgb(1., 1., 0.)),
                    MoneyComp,
                )
            ],
        ))
        .id();

    let experience = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: px(10),
                ..default()
            },
            children![
                (Text::new("Exp:"), TextColor::from(Color::BLACK)),
                (
                    Text::new(experience.0.to_string()),
                    TextColor::from(Color::linear_rgb(1., 0., 0.)),
                    ExperienceComp,
                )
            ],
        ))
        .id();

    commands.entity(container).add_child(gold);
    commands.entity(container).add_child(experience);
}

pub fn collectable_system(
    money: Res<Money>,
    experience: Res<Experience>,
    mut text_query: ParamSet<(
        Single<&mut Text, With<MoneyComp>>,
        Single<&mut Text, With<ExperienceComp>>,
    )>,
) {
    let mut money_text = text_query.p0().into_inner();
    money_text.0 = money.0.to_string();

    let mut experience_text = text_query.p1().into_inner();
    experience_text.0 = experience.0.to_string();
}
