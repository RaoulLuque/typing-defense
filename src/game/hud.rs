mod systems;
use systems::*;

pub mod components;
use components::*;

use crate::menu::{GameStartedState, MenuState};

use super::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<InGameHudUiElement>()
            .register_type::<InGameHudParent>()
            .register_type::<ScoreText>()
            .register_type::<WpmText>()
            // Add startup systems
            .add_systems(Startup, spawn_hud)
            .add_systems(
                OnEnter(RoundState::InBetweenRounds),
                spawn_in_between_rounds_text.run_if(
                    in_state(GameStartedState::GameHasStarted)
                        .and_then(in_state(MenuState::NotInTheMenu)),
                ),
            )
            .add_systems(
                OnExit(RoundState::InBetweenRounds),
                crate::menu::systems::despawn_entities_with_specific_component::<
                    InBetweenRoundsHudUiElement,
                >,
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
            )
            .add_systems(
                Update,
                update_round_number_hud_element
                    .after(super::rounds_and_indicators::systems::increase_round_counter),
            );
    }
}
