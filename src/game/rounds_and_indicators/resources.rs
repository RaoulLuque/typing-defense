use super::*;
use std::fmt;

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

/// Resource for tracking the number of enemies that have been spawned this round.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct NumberOfEnemiesSpawnedThisRound {
    pub number: u32,
}

/// Resource for tracking the number of enemies that have unlived this round
/// e.g. have been typed, ran out of screen or ran into the castle.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct NumberOfEnemiesUnlivedThisRound {
    pub number: u32,
}

/// Resource for tracking the number of enemies that have been typed this round.
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct NumberOfEnemiesTypedThisRound {
    pub number: u32,
}

/// Base speed of enemies in the current round.
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
pub struct RoundNumber {
    pub number: u32,
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

/// Resource for tracking the number of enemies that are supposed to be spawned this round
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct WordPerMinuteTypedIndicator {
    pub wpm: f64,
}

impl Default for WordPerMinuteTypedIndicator {
    fn default() -> WordPerMinuteTypedIndicator {
        WordPerMinuteTypedIndicator { wpm: 0.0 }
    }
}

/// Resource for tracking the score. For score calculation see [`super::systems::update_score`]
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct ScoreIndicator {
    pub score: u64,
}

impl Default for ScoreIndicator {
    fn default() -> ScoreIndicator {
        ScoreIndicator { score: 0 }
    }
}

/// Resource for tracking streaks.
///
/// Counts up for each letter typed and resets when a mistake is made (mistakes do not count if
/// nothing is currently being typed and letter is pressed that doesn't belong to any enemy)
/// or enemy runs into castle or out of screen.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct StreakIndicator {
    pub number: u64,
}

impl Default for StreakIndicator {
    fn default() -> StreakIndicator {
        StreakIndicator { number: 0 }
    }
}

/// Resource for tracking streaks (typing without mistakes and no enemy hitting the castle)
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct DifficultyIndicator {
    pub difficulty: Difficulty,
}

impl Default for DifficultyIndicator {
    fn default() -> DifficultyIndicator {
        DifficultyIndicator {
            difficulty: Difficulty::default(),
        }
    }
}

#[derive(Default, Reflect)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Difficulty::Easy => "Easy",
                Difficulty::Hard => "Hard",
                Difficulty::Medium => "Medium",
            }
        )
    }
}
