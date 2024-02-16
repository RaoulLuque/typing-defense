mod systems;

use systems::*;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowTheme},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

fn main() {
    App::new()
        // Add default plugin and tweak ImagePlugin for smoother Animations with SpriteSheets and
        // window plugin
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        title: "Typing Defense".to_string(),
                        resizable: true,
                        window_theme: Some(WindowTheme::Dark),
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..Default::default()
                    }),
                    ..default()
                }),
        )
        // Add Debugging info in game
        .add_plugins(WorldInspectorPlugin::new())
        // Spawn camera and add background
        .add_systems(Startup, spawn_background)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, toggle_borderless_fullscreen)
        .run();
}
