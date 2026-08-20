use bevy::prelude::*;

use crate::{
    consts::*,
    hud::{
        MenuNavigationButton,
        menu::{ButtonPressed, FrameButton},
    },
    resource_paths,
};

pub fn menu_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    comp: impl Component,
    resource_path: String,
) -> Entity {
    let frame = frame_button(commands, asset_server, comp);

    let icon = commands
        .spawn(icon_button(asset_server, resource_path))
        .id();
    commands.entity(frame).add_child(icon);

    return frame;
}

pub fn place_holder<T: Component>(commands: &mut Commands, comp: Option<T>) -> Entity {
    let mut node = commands.spawn(default_node());
    if let Some(comp) = comp {
        node.insert(comp);
    }
    return node.id();
}

pub fn arrow_node(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    menu_button(
        commands,
        asset_server,
        MenuNavigationButton::Back,
        resource_paths::load_nav_icons(MenuNavigationButton::Back),
    )
}

pub fn frame_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    comp: impl Component,
) -> Entity {
    commands
        .spawn((
            default_node(),
            ImageNode::from(asset_server.load("menu/frame.png")),
            FrameButton,
            Button,
            ButtonPressed(false),
            comp,
        ))
        .id()
}

pub fn icon_button(asset_server: &Res<AssetServer>, resource_path: String) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        ImageNode::from(asset_server.load(resource_path)),
    )
}

fn default_node() -> Node {
    Node {
        width: px(ACTION_BUTTON_SIZE.x),
        height: px(ACTION_BUTTON_SIZE.y),
        border: UiRect::all(px(5)),
        border_radius: BorderRadius::ZERO,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
