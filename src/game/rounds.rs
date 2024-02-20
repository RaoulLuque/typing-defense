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
            // Initialize Resources
            .init_resource::<MaxNumberOfEnemiesCurrentRound>()
            .init_resource::<NumberOfEnemiesSpawnedCurrentRound>()
            .init_resource::<EnemyBaseSpeedCurrentRound>()
            .init_resource::<NumberOfEnemiesTypedCurrentRound>()
            .init_resource::<RoundCounter>()
            // Add systems that run on entry of round
            .add_systems(OnEnter(RoundState::InRound), increase_round_difficulty)
            .add_systems(OnEnter(RoundState::InRound), increase_round_counter)
            // Add update systems
            .add_systems(
                Update,
                check_if_round_is_over.run_if(in_state(RoundState::InRound)),
            )
            .add_systems(
                Update,
                proceed_to_next_round_from_in_between_rounds
                    .run_if(in_state(RoundState::InBetweenRounds)),
            );
    }
}
