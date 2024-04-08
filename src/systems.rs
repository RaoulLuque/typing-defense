use bevy::{prelude::*, window::PrimaryWindow};

/// Spawns the camera
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Spawns the background asset
pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        texture: asset_server.load("background/background_new.png"),
        ..default()
    },));
}

/// Enables the option to press F11 for toggling between window modes borderless fullscreen and windowed
pub fn toggle_borderless_fullscreen(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
