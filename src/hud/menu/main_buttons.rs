use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    hud::{
        MenuActionButton, MenuNavigationButton,
        menu::component::{frame_button, icon_button},
    },
    resource_paths,
};

pub fn navigation_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Vec<Entity> {
    let mut res = Vec::new();

    for m_button in MenuNavigationButton::iter() {
        if m_button == MenuNavigationButton::Back {
            continue;
        }
        let frame = frame_button(commands, asset_server, m_button);

        let icon = commands
            .spawn(icon_button(
                asset_server,
                resource_paths::load_nav_icons(m_button),
            ))
            .id();

        if [MenuNavigationButton::SelTurret].contains(&m_button) {
            let text_over_icon = text_icon(commands, "$");
            commands.entity(icon).add_child(text_over_icon);
        }

        commands.entity(frame).add_child(icon);
        res.push(frame);
    }

    return res;
}

pub fn action_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Vec<Entity> {
    let mut res = Vec::new();

    for action_button in MenuActionButton::iter() {
        let frame = frame_button(commands, asset_server, action_button);

        let icon = commands
            .spawn(icon_button(
                asset_server,
                resource_paths::load_action_icons(action_button),
            ))
            .id();

        if [MenuActionButton::UpgradeBase].contains(&action_button) {
            let text = text_icon(commands, "+");
            commands.entity(icon).add_child(text);
        }

        commands.entity(frame).add_child(icon);
        res.push(frame);
    }

    return res;
}

fn text_icon(commands: &mut Commands, text: &str) -> Entity {
    commands
        .spawn((
            Node {
                margin: UiRect::all(px(8.)),
                ..default()
            },
            Text::new(text),
            TextColor::from(Color::linear_rgb(0., 1., 0.)),
            TextFont {
                font_size: 20.,
                ..default()
            },
        ))
        .id()
}
