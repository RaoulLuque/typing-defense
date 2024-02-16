use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        texture: asset_server.load("background/parchmentAncientUpscaled.png"),
        ..default()
    },));
}

pub fn toggle_borderless_fullscreen(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        let mut window = window_query
            .get_single_mut()
            .expect("Primary window should exist");
        window.mode = match window.mode {
            bevy::window::WindowMode::BorderlessFullscreen => bevy::window::WindowMode::Windowed,
            _ => bevy::window::WindowMode::BorderlessFullscreen,
        }
    }
}
