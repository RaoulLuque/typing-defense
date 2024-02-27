pub mod components;
use components::*;

mod systems;
use systems::*;

use super::*;

pub struct DecorationsPlugin;

impl Plugin for DecorationsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<Tree>()
            .register_type::<StaticDecorationType>()
            // Initialize Resources
            // Add systems for startup into the game
            .add_systems(Startup, spawn_trees)
            // Add update systems
            .add_systems(Update, animate_trees);
    }
}
