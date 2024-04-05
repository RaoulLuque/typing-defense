mod systems;
use systems::*;

pub mod game;
use game::GamePlugin;

mod menu;
use menu::MenuPlugin;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

fn main() {
    let mut app = App::new();
    app
        // Workaround for some wasm stuff
        .insert_resource(bevy::asset::AssetMetaCheck::Never)
        // Add default plugin and tweak ImagePlugin for smoother Animations with SpriteSheets and
        // window plugin
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        title: "Typing Defense".to_string(),
                        resizable: true,
                        window_theme: Some(WindowTheme::Dark),
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..Default::default()
                    }),
                    ..default()
                }),
        )
        // Initialize AppState
        .init_state::<AppState>()
        // Spawn camera and add background
        .add_systems(Startup, spawn_background)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, toggle_borderless_fullscreen)
        // Add game and menu plugins
        .add_plugins((GamePlugin, MenuPlugin));

    if cfg!(debug_assertions) {
        // Add Debugging info in game
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run()
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}
