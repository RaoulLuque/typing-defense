pub mod resources;
use resources::*;

pub mod systems;
use systems::*;

use crate::menu::MenuState;

use super::*;

pub struct RoundsAndIndicatorsPlugin;

impl Plugin for RoundsAndIndicatorsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<MaxNumberOfEnemiesCurrentRound>()
            .register_type::<NumberOfEnemiesSpawnedThisRound>()
            .register_type::<EnemyBaseSpeedCurrentRound>()
            .register_type::<NumberOfEnemiesUnlivedThisRound>()
            .register_type::<NumberOfEnemiesTypedThisRound>()
            .register_type::<RoundNumber>()
            .register_type::<RoundStopwatch>()
            .register_type::<WordPerMinuteTypedIndicator>()
            .register_type::<ScoreIndicator>()
            .register_type::<StreakIndicator>()
            .register_type::<DifficultyIndicator>()
            // Initialize Resources
            .init_resource::<MaxNumberOfEnemiesCurrentRound>()
            .init_resource::<NumberOfEnemiesSpawnedThisRound>()
            .init_resource::<EnemyBaseSpeedCurrentRound>()
            .init_resource::<NumberOfEnemiesUnlivedThisRound>()
            .init_resource::<NumberOfEnemiesTypedThisRound>()
            .init_resource::<RoundNumber>()
            .init_resource::<RoundStopwatch>()
            .init_resource::<WordPerMinuteTypedIndicator>()
            .init_resource::<ScoreIndicator>()
            .init_resource::<StreakIndicator>()
            .init_resource::<DifficultyIndicator>()
            // Add systems that run on entry of round
            .add_systems(
                OnEnter(RoundState::InRound),
                (
                    reset_indicators,
                    increase_round_difficulty,
                    increase_round_counter,
                    reset_round_stopwatch,
                    reset_wpm,
                ),
            )
            // Add update systems
            .add_systems(
                Update,
                tick_round_stopwatch.in_set(super::InputHandlingSystemSet::BeforeInputHandling),
            )
            .add_systems(
                Update,
                check_if_round_is_over.in_set(super::InputHandlingSystemSet::AfterInputHandling),
            )
            .add_systems(
                Update,
                proceed_to_next_round_from_in_between_rounds
                    .run_if(in_state(RoundState::InBetweenRounds)),
            )
            .add_systems(
                Update,
                (update_wpm).in_set(super::InputHandlingSystemSet::AfterInputHandling),
            )
            // Score needs to be updated after wpm and other indicators
            .add_systems(
                Update,
                update_score
                    .after(super::InputHandlingSystemSet::AfterInputHandling)
                    .run_if(
                        in_state(LoosingState::NotLost)
                            .and_then(in_state(AppState::InGame))
                            .and_then(in_state(MenuState::NotInTheMenu)),
                    ),
            )
            .add_systems(
                Update,
                update_number_of_enemies_typed
                    .after(super::InputHandlingSystemSet::AfterInputHandling)
                    .run_if(in_state(AppState::InGame).and_then(in_state(MenuState::NotInTheMenu))),
            )
            .add_systems(
                Update,
                (set_states_on_restart, reset_score_and_indicators_on_restart),
            );
    }
}
