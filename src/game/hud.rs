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
            .register_type::<InGameHudUiElement>()
            // Add systems for entering game
            .add_systems(
                OnEnter(super::AppState::InGame),
                (
                    spawn_wpm_hud_element,
                    spawn_score_hud_element,
                    spawn_streak_hud_element,
                ),
            )
            // Add update systems
            .add_systems(
                Update,
                update_score_hud_element.after(super::rounds_and_indicators::systems::update_score),
            )
            .add_systems(
                Update,
                update_streak_hud_element.in_set(super::InputHandlingSystemSet::AfterInputHandling),
            )
            .add_systems(
                Update,
                update_wpm_hud_element.after(super::rounds_and_indicators::systems::update_wpm),
            );
    }
}
