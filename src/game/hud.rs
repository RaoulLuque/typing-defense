pub mod resources;
use resources::*;

mod systems;
use systems::*;

pub mod components;
use components::*;

use super::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<WordPerMinuteTypedIndicator>()
            .register_type::<InRoundHudUiElement>()
            // Initialize Resources
            .init_resource::<WordPerMinuteTypedIndicator>()
            // Add systems for entering rounds
            .add_systems(OnEnter(super::RoundState::InRound), reset_wpm)
            .add_systems(OnEnter(super::AppState::InGame), spawn_wpm_hud_element)
            // Add update systems
            .add_systems(
                Update,
                update_wpm.in_set(super::InputHandlingSystemSet::AfterInputHandling),
            )
            .add_systems(Update, update_wpm_hud_element.after(update_wpm));
    }
}
