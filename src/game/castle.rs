mod components;
use components::*;
mod systems;
use systems::*;

use super::*;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<Castle>()
            // Initialize Resources
            // Setup list of words as asset
            .add_systems(OnEnter(AppState::InGame), spawn_castle);
        // Add update systems
    }
}
