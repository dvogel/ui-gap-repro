use bevy::{
    prelude::*,
    ui::{BackgroundColor, Val},
};

pub fn inject_ui_entities(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Hello world!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(0.0),
                    bottom: Val::Percent(1.0),
                },
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        BackgroundColor(Color::rgba(164.0 / 256.0, 116.0 / 256.0, 73.0 / 256.0, 0.5)),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("UI Gap Repro"),
                width: 1200.0,
                height: 600.0,
                ..Default::default()
            },
            exit_on_all_closed: true,
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(inject_ui_entities)
        .run();
}
