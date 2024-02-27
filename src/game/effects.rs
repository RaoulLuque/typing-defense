pub mod components;
use components::*;

mod systems;
use systems::*;

use super::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<Explosion>()
            // Initialize Resources
            // Add systems for startup into the game
            // Add update systems
            .add_systems(Update, animate_explosions);
    }
}
