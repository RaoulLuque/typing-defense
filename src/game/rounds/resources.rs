use super::*;

/// Initial speed of enemies at start of game
pub const INITIAL_ENEMY_SPEED: f32 = 30.0;
/// Number of enemies in the first round - super::systems::NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT
pub const INITIAL_MAX_NUMBER_OF_ENEMIES: u32 = 2;
use bevy::time::Stopwatch;
// Initial interval for spawning enemies
use enemies::resources::INITIAL_ENEMY_SPAWN_INTERVAL;

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

/// Base speed of enemies in the current round
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct EnemyBaseSpawnIntervalRound {
    pub interval: f32,
}

impl Default for EnemyBaseSpawnIntervalRound {
    fn default() -> EnemyBaseSpawnIntervalRound {
        EnemyBaseSpawnIntervalRound {
            interval: INITIAL_ENEMY_SPAWN_INTERVAL,
        }
    }
}

/// Counts up right at the start of the round. In between rounds the counter is the number of the round before.
///
/// Is zero in main menu and increments to 1 when pressing start game.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct RoundCounter {
    pub counter: u32,
}

/// Stopwatch for counting how much time has passed this round.
///
/// Is reset to 0 at beginning of each round.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct RoundStopwatch {
    pub stopwatch: Stopwatch,
}

impl Default for RoundStopwatch {
    fn default() -> RoundStopwatch {
        RoundStopwatch {
            stopwatch: Stopwatch::new(),
        }
    }
}
