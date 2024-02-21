pub mod components;
use components::*;

mod systems;
use systems::*;

use super::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app;
        // Register types for debug
        // Initialize Resources
        // Add systems for startup into the game
        // Add update systems
    }
}
