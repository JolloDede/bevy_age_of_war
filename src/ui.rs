use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, setup_buttons);
    }
}

#[derive(Component)]
struct UICamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), UICamera));
}

fn setup_buttons(mut commands: Commands) {
    commands.spawn((
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::MAX,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        children![
            Text::new("Button"),
            TextColor::default(),
            TextShadow::default()
        ],
    ));
}
