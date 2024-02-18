use super::systems::Words;
use bevy::prelude::*;

// Interval for checking if enemies should spawn in seconds
const ENEMY_SPAWN_TIMER: f32 = 1.0;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct IsSomethingBeingTyped {
    pub indicator: bool,
}

impl Default for IsSomethingBeingTyped {
    fn default() -> IsSomethingBeingTyped {
        IsSomethingBeingTyped { indicator: false }
    }
}

#[derive(Resource)]
pub struct WordsHandle(pub Handle<Words>);
