pub mod resources;
use resources::*;

pub mod systems;
use systems::*;

use super::*;

pub struct RoundsPlugin;

impl Plugin for RoundsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<MaxNumberOfEnemiesCurrentRound>()
            .register_type::<NumberOfEnemiesSpawnedCurrentRound>()
            .register_type::<EnemyBaseSpeedCurrentRound>()
            .register_type::<NumberOfEnemiesTypedCurrentRound>()
            .register_type::<RoundCounter>()
            .register_type::<RoundStopwatch>()
            // Initialize Resources
            .init_resource::<MaxNumberOfEnemiesCurrentRound>()
            .init_resource::<NumberOfEnemiesSpawnedCurrentRound>()
            .init_resource::<EnemyBaseSpeedCurrentRound>()
            .init_resource::<NumberOfEnemiesTypedCurrentRound>()
            .init_resource::<RoundCounter>()
            .init_resource::<RoundStopwatch>()
            // Add systems that run on entry of round
            .add_systems(OnEnter(RoundState::InRound), increase_round_difficulty)
            .add_systems(OnEnter(RoundState::InRound), increase_round_counter)
            .add_systems(OnEnter(super::RoundState::InRound), reset_round_stopwatch)
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
            );
    }
}
