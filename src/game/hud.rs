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
            .register_type::<InGameHudParent>()
            .register_type::<UiFixedZ>()
            .register_type::<ScoreText>()
            .register_type::<WpmText>()
            // Add startup systems
            .add_systems(Startup, spawn_hud)
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
