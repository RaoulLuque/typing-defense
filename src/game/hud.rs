pub mod resources;
use resources::*;

mod systems;
use systems::*;

use super::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<WordPerMinuteTypedIndicator>()
            // Initialize Resources
            .init_resource::<WordPerMinuteTypedIndicator>()
            // Add systems for entering rounds
            .add_systems(OnEnter(super::RoundState::InRound), reset_wpm)
            // Add update systems
            .add_systems(
                Update,
                update_wpm.in_set(super::InputHandlingSystemSet::AfterInputHandling),
            );
    }
}
