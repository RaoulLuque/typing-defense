use super::*;

/// Initial speed of enemies at start of game
const INITIAL_ENEMY_SPEED: f32 = 30.0;
/// Number of enemies in the first round - super::systems::NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT
const INITIAL_MAX_NUMBER_OF_ENEMIES: u32 = 2;

/// Resource for tracking the number of enemies that are supposed to be spawned this round
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct MaxNumberOfEnemiesCurrentRound {
    pub number: u32,
}

impl Default for MaxNumberOfEnemiesCurrentRound {
    fn default() -> MaxNumberOfEnemiesCurrentRound {
        MaxNumberOfEnemiesCurrentRound {
            number: INITIAL_MAX_NUMBER_OF_ENEMIES,
        }
    }
}

/// Resource for tracking the number of enemies that have been spawned this round
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct NumberOfEnemiesSpawnedCurrentRound {
    pub number: u32,
}

/// Resource for tracking the number of enemies that have been typed this round
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct NumberOfEnemiesTypedCurrentRound {
    pub number: u32,
}

/// Base speed of enemies in the current round
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct EnemyBaseSpeedCurrentRound {
    pub speed: f32,
}

impl Default for EnemyBaseSpeedCurrentRound {
    fn default() -> EnemyBaseSpeedCurrentRound {
        EnemyBaseSpeedCurrentRound {
            speed: INITIAL_ENEMY_SPEED,
        }
    }
}

/// Counts up right at the start of the round.
/// In between rounds the counter is the number of the round before.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct RoundCounter {
    pub counter: u32,
}
